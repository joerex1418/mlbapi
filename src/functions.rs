#[allow(unused)]
#[allow(dead_code)]

use pyo3::IntoPy;

use crate::{
    objects::people::{PersonResponse, Person},
    objects::schemas::schedule::ScheduleResponse,
    objects::schemas::team::TeamResponse
};
use crate::objects::rosters::RosterResponse;

#[allow(unused)]
pub fn get_team(team_id: usize) {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}",

        team_id = team_id.to_string()
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let json_resp: reqwest::Result<TeamResponse> = response.json();

        if let Ok(json_resp) = json_resp {
            println!("{:#?}", json_resp);
        }
    }
}

#[allow(unused)]
pub fn get_person(person_id: usize) -> Option<Person> {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/people/{person_id}",
        person_id = person_id.to_string()
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let json_resp: reqwest::Result<PersonResponse> = response.json();

        if let Ok(person_resp_obj) = json_resp {
            if let Some(person_obj) = person_resp_obj.people.get(0) {
                return Some(person_obj.to_owned());
            };
        };
    };

    None



}

#[allow(unused)]
pub fn get_roster(team_id: usize) -> Option<RosterResponse> {
    
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}/roster",
        team_id = team_id.to_string()
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let json_resp: reqwest::Result<RosterResponse> = response.json();
        if let Ok(roster_obj) = json_resp {
            return Some(roster_obj)
        }
    }

    None

}

#[allow(unused)]
pub fn get_schedule(date: Option<String>) -> Option<ScheduleResponse> {

    let url = match date {
        Some(date) => {
            format!(
                "https://statsapi.mlb.com/api/v1/schedule?sportId=1&date={}",
                date.to_string()
            )
        }
        None => {
            "https://statsapi.mlb.com/api/v1/schedule?sportId=1".to_string()
        }
    };
    
    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        
        let json_resp: reqwest::Result<ScheduleResponse> = response.json();
        if let Ok(json_resp) = json_resp {
            return Some(json_resp)
        } 
    } 

    return None
    

}
