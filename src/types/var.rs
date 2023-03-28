use std::string;

use bson::oid::ObjectId;
use serde::{
    Deserialize,
    Serialize,
};





pub fn get_message() -> String {
    String::from("message from function")
}


#[derive (Deserialize,Debug, Clone)]
pub struct GetConnector{
    pub name:String,
    pub description:String,
    pub email:String,
    pub api_key:String,
    pub bot_type:String,
    pub active:bool,
    pub token:String,
    pub chatid:String,
    pub project:Vec<ProjectID>,
    pub event:Vec<String>,
}

#[derive (Serialize,Deserialize,Debug, Clone)]
pub struct UserSetting{
    pub name:String,
    pub description:String,
    pub email:String,
    pub api_key:String,
    pub bot_type:String,
    pub active:bool,
    pub token:String,
    pub chatid:String,
    pub project:Vec<ProjectID>,
    pub event:Vec<String>,
}

#[derive (Deserialize,Debug, Clone)]
pub struct  CardHomePage{
    pub card:Vec<GetConnector>,
}

#[derive (Serialize,Deserialize,Debug, Clone)]
pub struct  ProjectID{
    pub id:String,
    pub name:String,
}

#[derive (Debug,Clone,PartialEq)]
pub struct  ProjectStat{
    pub id:String,
    pub name:String,
    pub status:bool,

}

#[derive (Serialize,Debug, Clone)]
pub struct NewConnector{
    pub name:String,
    pub description:String,
    pub email:String,
    pub api_key:String,
    pub bot_type:String,
    pub active:bool,
    pub token:String,
    pub chatid:String,
    pub project:Vec<String>,
    pub event:Vec<String>,
}

pub struct MsgErr {
    pub header:String,
    pub body:String,
}

// ini area farel

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Users {
    pub name: String,
    pub description: String,
    pub cloudSessionToken: String,
    pub platformApiKey: String,
    pub platformEmail: String,
    pub platformType: String,
    pub schedule: i64,
    pub lastActive: i64,
    pub active: bool,
    pub checkActiveStatus: bool,
    pub checkDoubleEmail: bool,
    pub checkDoubleName: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectId{
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsersData {
    #[serde(rename = "_id")]
    pub id: ProjectId,
    pub name: String,
    pub description: String,
    pub platformEmail: String,
    pub platformApiKey: String,
    pub platformType: String,
    pub cloudSessionToken: String,
    pub active: bool,
    pub schedule: i64,
    pub lastActive: i64,
    pub checkDoubleName: bool,
    pub checkDoubleEmail: bool,
    pub checkActiveStatus: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectList {
    pub list: Option<Vec<UsersData>>,
    pub world: Option<String>,
    pub error_description: Option<String>,
    // other_data: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Schedule {
    pub task: String,
    pub superhero: String,
    pub is_on_going: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SchedulesData {
    pub list: Option<Vec<Schedule>>,
    pub world: Option<String>,
    pub error_description: Option<String>,
    // other_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortReturnValueInner{
    #[serde(rename = "$oid")]
    id: String, }

#[derive(Debug, Serialize, Deserialize)]
pub struct PostReturnValue {
    insertedId: PortReturnValueInner
}

#[derive(Serialize, Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub status: String,
}