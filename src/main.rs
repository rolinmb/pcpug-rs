use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::fmt;

const BASEURL: &str = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/";
const RETURNTYPE: &str = "/JSON";

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
    bonds: Option<BondInfo>,
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("\nmain() :: ERROR -> Please enter compound names (in alphabetical characters) as arguments to cargo run");
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
        let status = response.status();
        if status.is_success() {
            println!("\nmain() :: Successfully requested PubChem PUG REST API\n\nStatus: {}\n", status);
            let json: serde_json::Value = response.json().await?;
            //println!("\nmain() :: Raw JSON response\n\n{}\n", json);
            if let Some(compounds) = json["PC_Compounds"].as_array() {
                for compound in compounds {
                    if let Ok(compound) = serde_json::from_value::<Compound>(compound.clone()) {
                        println!("{}", compound);
                    }
                }
            } else {
                eprintln!("\nmain() :: ERROR -> Could not find any compound using query name '{}'; or parsing error potentially occurred\n\nStatus: {}\n", name, status);
            }
        } else {
            eprintln!("\nmain() :: ERROR -> Failed to fetch compound using query name '{}'\n\nStatus: {}\n", name, status);
        }
    }
    Ok(())
}