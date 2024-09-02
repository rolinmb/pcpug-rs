//use reqwest::Error;
use serde::Deserialize;
use plotters::prelude::*;
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;

const BASEURL: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/";
const RETURNTYPE: &str = "/JSON";
const PNGDIR: &str = "png_out/";

#[derive(Debug, Deserialize)]
struct AtomInfo {
    aid: Vec<u32>,
    element: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct BondInfo {
    aid1: Vec<u32>,
    aid2: Vec<u32>,
    order: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct Conformer {
    x: Vec<f64>,
    y: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct Coords {
    conformers: Vec<Conformer>,
}

#[derive(Debug, Deserialize)]
struct Props {
    urn: Urn,
    value: PropertyValue,
}

#[derive(Debug, Deserialize)]
struct Stereo {
    tetrahedral: Option<Tetrahedral>,
}

#[derive(Debug, Deserialize)]
struct Tetrahedral {
    above: u32,
    below: u32,
    bottom: u32,
    center: u32,
    parity: u32,
    top: u32,
    #[serde(rename = "type")]
    ttype: u32,
}

#[derive(Debug, Deserialize)]
struct Urn {
    datatype: Option<u32>,
    label: Option<String>,
    name: Option<String>,
    release: Option<String>,
    software: Option<String>,
    source: Option<String>,
    version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PropertyValue {
    fval: Option<f64>,
    ival: Option<u32>,
    sval: Option<String>,
    binary: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IUPACName {
    #[serde(rename = "IUPACName")]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Compound {
    cid: Option<u32>,
    atoms: Option<AtomInfo>,
    bonds: Option<Vec<BondInfo>>,
    coords: Option<Vec<Coords>>,
    props: Option<Vec<Props>>,
    stereo: Option<Vec<Stereo>>,
    molecular_formula: Option<String>,
    molecular_weight: Option<f64>,
    inchi: Option<String>,
    inchikey: Option<String>,
    isomeric_smiles: Option<String>,
    tpsa: Option<f64>,
    xlogp: Option<f64>,
    exact_mass: Option<f64>,
    complexity: Option<f64>,
    h_bond_donor_count: Option<u32>,
    h_bond_acceptor_count: Option<u32>,
    rotatable_bond_count: Option<u32>,
    heavy_atom_count: Option<u32>,
    charge: Option<i32>,
    iupac: Option<Vec<IUPACName>>,
}

impl fmt::Display for Compound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Compound Information:")?;
        writeln!(f, "---------------------")?;
        writeln!(f, "CID: {:?}", self.cid)?;
        writeln!(f, "Molecular Formula: {:?}", self.molecular_formula)?;
        writeln!(f, "Molecular Weight: {:?}", self.molecular_weight)?;
        writeln!(f, "InChI: {:?}", self.inchi)?;
        writeln!(f, "InChIKey: {:?}", self.inchikey)?;
        writeln!(f, "Isomeric SMILES: {:?}", self.isomeric_smiles)?;
        writeln!(f, "TPSA: {:?}", self.tpsa)?;
        writeln!(f, "XLogP: {:?}", self.xlogp)?;
        writeln!(f, "Exact Mass: {:?}", self.exact_mass)?;
        writeln!(f, "Complexity: {:?}", self.complexity)?;
        writeln!(f, "H-Bond Donor Count: {:?}", self.h_bond_donor_count)?;
        writeln!(f, "H-Bond Acceptor Count: {:?}", self.h_bond_acceptor_count)?;
        writeln!(f, "Rotatable Bond Count: {:?}", self.rotatable_bond_count)?;
        writeln!(f, "Heavy Atom Count: {:?}", self.heavy_atom_count)?;
        writeln!(f, "Charge: {:?}", self.charge)?;
        writeln!(f, "---------------------")?;
        if let Some(atoms) = &self.atoms {
            writeln!(f, "Atoms:")?;
            for (i, element) in atoms.element.iter().enumerate() {
                writeln!(f, "  Atom ID: {}, Element: {}", atoms.aid[i], element)?;
            }
        } else {
            writeln!(f, "Atoms: None")?;
        }
        writeln!(f, "---------------------")?;
        if let Some(bonds) = &self.bonds {
            writeln!(f, "Bonds:")?;
            for i in 0..bonds.aid1.len() {
                writeln!(
                    f,
                    "  Bond {}: Atom1 {}, Atom2 {}, Order {}",
                    i + 1,
                    bonds.aid1[i],
                    bonds.aid2[i],
                    bonds.order[i]
                )?;
            }
        } else {
            writeln!(f, "Bonds: None")?;
        }
        writeln!(f, "---------------------")?;
        if let Some(coords) = &self.coords {
            writeln!(f, "Coordinates:")?;
            for (i, conformer) in coords.iter().flat_map(|c| c.conformers.iter()).enumerate() {
                writeln!(
                    f,
                    "  Conformer {}:\\nX = {:?},\nY = {:?}",
                    i + 1,
                    conformer.x,
                    conformer.y
                )?;
            }
        } else {
            writeln!(f, "Coordinates: None")?;
        }
        writeln!(f, "---------------------")?;
        if let Some(props) = &self.props {
            writeln!(f, "Properties:")?;
            for prop in props {
                writeln!(
                    f,
                    "  Property: {:?}, Value: {:?}",
                    prop.urn.label, prop.value
                )?;
            }
        } else {
            writeln!(f, "Properties: None")?;
        }
        writeln!(f, "---------------------")?;
        if let Some(stereo) = &self.stereo {
            writeln!(f, "Stereo Information:")?;
            for (i, s) in stereo.iter().enumerate() {
                if let Some(tetrahedral) = &s.tetrahedral {
                    writeln!(
                        f,
                        "  Stereo {}: Above = {}, Below = {}, Bottom = {}, Center = {}, Parity = {}, Top = {}, Type = {}",
                        i + 1,
                        tetrahedral.above,
                        tetrahedral.below,
                        tetrahedral.bottom,
                        tetrahedral.center,
                        tetrahedral.parity,
                        tetrahedral.top,
                        tetrahedral.ttype
                    )?;
                }
            }
        } else {
            writeln!(f, "Stereo Information: None")?;
        }
        writeln!(f, "---------------------")?;
        if let Some(iupac) = &self.iupac {
            writeln!(f, "IUPAC Names:")?;
            for name in iupac {
                writeln!(f, "  {}", name.name.as_deref().unwrap_or("Unknown"))?;
            }
        } else {
            writeln!(f, "IUPAC Names: None")?;
        }
        Ok(())
    }
}

fn plot_molecule(name: &str, coords: &[Coords], bonds: &[BondInfo]) -> Result<(), Box<dyn std::error::Error>> {
    let png_name = format!("{}{}.png", PNGDIR, name);
    let root = BitMapBackend::new(&png_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(name, ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..10f64, 0f64..10f64)?;
    chart.configure_mesh().draw()?;
    if let Some(conformer) = coords.get(0).and_then(|c| c.conformers.get(0)) {
        for bond in bonds {
            for i in 0..bond.aid1.len() {
                let atom1_idx = bond.aid1[i] as usize;
                let atom2_idx = bond.aid2[i] as usize;
                chart.draw_series(LineSeries::new(
                    vec![
                        (conformer.x[atom1_idx], conformer.y[atom1_idx]),
                        (conformer.x[atom2_idx], conformer.y[atom2_idx]),
                    ],
                    &RED,
                ))?;
            }
        }
    } else {
        eprintln!("No conformers available for plotting.");
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(PNGDIR);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("src/main.rs usage :: cargo run <compound_name> <compound_name> <compound_name> ... (at least one additional argument)");
        return Ok(());
    }
    let names: Vec<String> = args.into_iter()
        .skip(1)
        .filter(|arg| !arg.chars().all(char::is_numeric))
        .collect();
    if names.is_empty() {
        eprintln!("\nmain() :: ERROR -> None of the compounds entered are purely alphabetical");
        return Ok(());
    }
    for name in &names {
        let url = format!("{}{}{}", BASEURL, name, RETURNTYPE);
        let response = reqwest::get(&url).await?;
        let compound: Compound = response.json().await?;
        println!("{}", compound);
        if let Some(coords) = compound.coords {
            if let Some(bonds) = compound.bonds {
                plot_molecule(name, &coords, &bonds)?;
            }
        }
    }
    Ok(())
}
