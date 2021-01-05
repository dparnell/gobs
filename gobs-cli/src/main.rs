extern crate dot_vox;
extern crate structopt;

use structopt::StructOpt;

use dot_vox::load;
use gobs::cubic_surface_extractor::extract_cubic_mesh;
use gobs::raw_volume::RawVolume;
use gobs::raw_volume_sampler::RawVolumeSampler;
use gobs::region::Region;
use gobs::volume::Volume;
use std::fs::File;
use std::io::{self, Error, Write};

#[derive(StructOpt, Debug)]
#[structopt(name = "gobs-cli")]
struct Options {
    #[structopt(name = "vox-file")]
    vox_file: String,

    #[structopt(name = "out-file")]
    output: Option<String>,
}

impl Options {
    fn get_output(&self) -> Result<Box<dyn Write>, Error> {
        match self.output {
            Some(ref path) => File::create(path).map(|f| Box::new(f) as Box<dyn Write>),
            None => Ok(Box::new(io::stdout())),
        }
    }
}

fn main() -> std::io::Result<()> {
    let options = Options::from_args();
    let mut out = options.get_output().unwrap();

    let vox_file = load(&options.vox_file).unwrap();

    let hex_palette = vox_file
        .palette
        .iter()
        .map(|rgba| {
            let a = (rgba >> 24) as u8;
            let b = (rgba >> 16) as u8;
            let g = (rgba >> 8) as u8;
            let r = *rgba as u8;

            format!("{{\"r\":{},\"g\":{},\"b\":{},\"a\":{}}}", r, g, b, a)
        })
        .collect::<Vec<String>>()
        .join(", ");

    writeln!(out, "{{")?;
    writeln!(out, " \"palette\": [{}],", hex_palette)?;
    writeln!(out, " \"models\": [")?;
    let mut first = true;
    for model in vox_file.models {
        let region = Region::sized(
            model.size.x as i32,
            model.size.y as i32,
            model.size.z as i32,
        );
        let mut volume = RawVolume::new(region);

        for voxel in model.voxels {
            volume
                .set_voxel_at(voxel.x as i32, voxel.y as i32, voxel.z as i32, voxel.i)
                .unwrap();
        }

        let mesh = extract_cubic_mesh(
            &mut RawVolumeSampler::new(&volume),
            &volume.valid_region,
            None,
            None,
        )
        .unwrap();

        let vertices = mesh
            .vertices()
            .iter()
            .map(|v| {
                let pos = v.decode();
                format!(
                    "{{\"x\":{},\"y\":{},\"z\":{},\"c\":{}}}",
                    pos.x, pos.y, pos.z, v.data
                )
            })
            .collect::<Vec<String>>()
            .join(",");

        let polygons = mesh
            .indices()
            .chunks(3)
            .map(|tri| format!("[{},{},{}]", tri[0], tri[1], tri[2]))
            .collect::<Vec<String>>()
            .join(",");

        if first {
            first = false;
        } else {
            writeln!(out, ",")?;
        }
        writeln!(out, "  {{")?;
        writeln!(out, "   \"vertices\": [{}],", vertices)?;
        writeln!(out, "   \"polygons\": [{}]", polygons)?;
        write!(out, "  }}")?;
    }
    writeln!(out, "\n ]")?;
    writeln!(out, "}}")?;

    Ok(())
}
