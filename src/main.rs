use crate::models::geoind::Geoindicator;
pub mod models;

use postgres::{ Client, NoTls };
// use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use std::env;



#[macro_use]
extern crate serde_derive;
extern crate chrono;

//DATABASE URL
const DB_URL: &str = env!("DATABASE_URL");

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

//main function
fn main() {

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8083")).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

//handle requests
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {

                r if r.starts_with("GET /geoindicators") => handle_get_all_request(r),

                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

//handle get all request
fn handle_get_all_request(_request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut geoindicators: Vec<Geoindicator> = Vec::new();

            for row in client.query("SELECT * FROM \"geoIndicators\";", &[]).unwrap() {
                geoindicators.push(Geoindicator {
                    rid: row.get(idx: 0),
                    ProjectKey: row.get(idx: 1),
                    PrimaryKey: row.get(idx: 2),
                    DateVisited: row.get(idx: 3),
                    EcologicalSiteId: row.get(idx: 4),
                    PercentCoveredByEcoSite: row.get(idx: 5),
                    Latitude_NAD83: row.get(idx: 6),
                    Longitude_NAD83: row.get(idx: 7),
                    LocationStatus: row.get(idx: 8),
                    LocationType: row.get(idx: 9),
                    Latitude_NAD83_Actual: row.get(idx: 10),
                    Longitude_NAD83_Actual: row.get(idx: 11),
                    BareSoilCover: row.get(idx: 12),
                    TotalFoliarCover: row.get(idx: 13),
                    AH_AnnGrassCover: row.get(idx: 14),
                    AH_ForbCover: row.get(idx: 15),
                    AH_GrassCover: row.get(idx: 16),
                    AH_PerenForbCover: row.get(idx: 17),
                    AH_PerenForbGrassCover: row.get(idx: 18),
                    AH_PerenGrassCover: row.get(idx: 19),
                    AH_ShrubCover: row.get(idx: 20),
                    FH_CyanobacteriaCover: row.get(idx: 21),
                    FH_DepSoilCover: row.get(idx: 22),
                    FH_DuffCover: row.get(idx: 23),
                    FH_EmbLitterCover: row.get(idx: 24),
                    FH_HerbLitterCover: row.get(idx: 25),
                    FH_LichenCover: row.get(idx: 26),
                    FH_MossCover: row.get(idx: 27),
                    FH_RockCover: row.get(idx: 28),
                    FH_TotalLitterCover: row.get(idx: 29),
                    FH_VagrLichenCover: row.get(idx: 30),
                    FH_WaterCover: row.get(idx: 31),
                    FH_WoodyLitterCover: row.get(idx: 32),
                    GapCover_101_200: row.get(idx: 33),
                    GapCover_200_plus: row.get(idx: 34),
                    GapCover_25_50: row.get(idx: 35),
                    GapCover_25_plus: row.get(idx: 36),
                    GapCover_51_100: row.get(idx: 37),
                    Hgt_Forb_Avg: row.get(idx: 38),
                    Hgt_Grass_Avg: row.get(idx: 39),
                    Hgt_Herbaceous_Avg: row.get(idx: 40),
                    Hgt_PerenForb_Avg: row.get(idx: 41),
                    Hgt_PerenForbGrass_Avg: row.get(idx: 42),
                    Hgt_PerenGrass_Avg: row.get(idx: 43),
                    Hgt_Woody_Avg: row.get(idx: 44),
                    RH_AnnualProd: row.get(idx: 45),
                    RH_BareGround: row.get(idx: 46),
                    RH_BioticIntegrity: row.get(idx: 47),
                    RH_CommentsBI: row.get(idx: 48),
                    RH_CommentsHF: row.get(idx: 49),
                    RH_CommentsSS: row.get(idx: 50),
                    RH_Compaction: row.get(idx: 51),
                    RH_DeadDyingPlantParts: row.get(idx: 52),
                    RH_FuncSructGroup: row.get(idx: 53),
                    RH_Gullies: row.get(idx: 54),
                    RH_HydrologicFunction: row.get(idx: 55),
                    RH_InvasivePlants: row.get(idx: 56),
                    RH_LitterAmount: row.get(idx: 57),
                    RH_LitterMovement: row.get(idx: 58),
                    RH_PedestalsTerracettes: row.get(idx: 59),
                    RH_PlantCommunityComp: row.get(idx: 60),
                    RH_ReprodCapabilityPeren: row.get(idx: 61),
                    RH_Rills: row.get(idx: 62),
                    RH_SoilSiteStability: row.get(idx: 63),
                    RH_SoilSurfLossDeg: row.get(idx: 64),
                    RH_SoilSurfResisErosion: row.get(idx: 65),
                    RH_WaterFlowPatterns: row.get(idx: 66),
                    RH_WindScouredAreas: row.get(idx: 67),
                    SoilStability_All: row.get(idx: 68),
                    SoilStability_Protected: row.get(idx: 69),
                    SoilStability_Unprotected: row.get(idx: 70),
                    mlra_name: row.get(idx: 71),
                    mlrarsym: row.get(idx: 72),
                    na_l1name: row.get(idx: 73),
                    na_l2name: row.get(idx: 74),
                    us_l3name: row.get(idx: 75),
                    us_l4name: row.get(idx: 76),
                    State: row.get(idx: 77),
                    modis_landcover: row.get(idx: 78),
                    DBKey: row.get(idx: 79),
                    DateLoadedInDb: row.get(idx: 80)
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&geoindicators).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}
