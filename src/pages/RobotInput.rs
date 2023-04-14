use yew::prelude::*;

use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use yew_router::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;
// use serde::{
//     Deserialize,
//     Serialize,
// };
use crate::router::route::AppRoute;
use crate::types::var::{
    UsersData,
    ProjectId,
    PostReturnValue,
    MsgErr
};



pub enum Msg {
    InputText(String),
    InputName(String),
    InputDesc(String),
    InputEmail(String),
    InputScheduler(String),
    InputApi(String),
    InputToken(String),
    InputSelect(String),
    InputActive(String),
    CheckDoubleName,
    CheckDoubleEmail,
    CheckActiveStatus,
    // Login,
    GetData(Vec<UsersData>),
    FetchData,
    // SelectProject,
    Ignore,
    FetchOne(UsersData),
    DeleteData,
    UpdateData,
    Home,
    SendData,
    RunProgram,
    doubleEmailThreshold(f32),
    doubleNameThreshold(f32),
    CheckSuccess,
    InvalidCredential,
    Success,
}

#[derive(Properties, Clone)]
pub struct idProjectProps{
    pub idProject : String,
}

pub struct RobotInput {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component

    // DATA
    username: String,
    status: String,
    project: Vec<UsersData>,
    data: UsersData,

    // SERVICES
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    idProject: String,
    msg_err:MsgErr,
}

impl Component for RobotInput {
    type Message = Msg;
    type Properties = idProjectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            // DATA
            username: String::from(""),
            status: String::from(""),

            // SERVICES
            link: link.clone(),
            fetch_task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            idProject: props.idProject,
            project:vec![],
            data: UsersData { id: ProjectId{
                oid: String::from("")
            },
                name: String::from(""),
                description: String::from(""),
                platformEmail: String::from(""),
                platformApiKey: String::from(""),
                // platformType: String::from(""),
                cloudSessionToken: String::from(""),
                active: false,
                schedule: 0,
                lastActive: 0,
                checkDoubleName: false,
                checkDoubleEmail: false,
                checkActiveStatus: false,
                doubleEmailThreshold: 100.0,
                doubleNameThreshold: 100.0,
                modified: None,
                created: None
            },
            msg_err:MsgErr { 
                header:"".to_string(),
                body:"".to_string(),
            },
        }
        
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                // FETCHING....
                ConsoleService::info(&format!("Test {:?}", self.idProject));
                let request = Request::get(format!{"https://atlassian-robot-api.dev-domain.site/robots?_id={}", self.idProject})
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<UsersData, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("status is {:?}", status_number));

                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::FetchOne(dataok)
                            }
                            Err(error) => {
                                ConsoleService::info("ignore.");
                                Msg::Ignore
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);


                true
            }
            Msg::DeleteData => {
                // FETCHING....
                ConsoleService::info(&format!("Test {:?}", self.idProject));
                let request = Request::delete(format!{"https://atlassian-robot-api.dev-domain.site/robots?_id={}", self.idProject})
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<Vec<UsersData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("status is {:?}", status_number));

                        if meta.status.is_success(){
                            {Msg::Home}
                        }else{
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("data response {:?}", &dataok));
                                    Msg::Home
                                }
                                Err(error) => {
                                    ConsoleService::info("ignore.");
                                    Msg::Ignore
                                }
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true
            }
            Msg::UpdateData => {

                let update = UsersData {
                    id: ProjectId{
                        oid: self.idProject.clone()
                    },
                    name: self.data.name.clone(),
                    description: self.data.description.clone(),
                    platformEmail: self.data.platformEmail.clone(),
                    platformApiKey: self.data.platformApiKey.clone(),
                    // platformType: self.data.platformType.clone(),
                    cloudSessionToken: self.data.cloudSessionToken.clone(),
                    active: self.data.active.clone(),
                    schedule: self.data.schedule.clone(),
                    lastActive: self.data.lastActive.clone(),
                    checkActiveStatus: self.data.checkActiveStatus.clone(),
                    checkDoubleName: self.data.checkDoubleName.clone(),
                    checkDoubleEmail: self.data.checkDoubleEmail.clone(),
                    doubleEmailThreshold: self.data.doubleEmailThreshold.clone(),
                    doubleNameThreshold: self.data.doubleNameThreshold.clone(),
                    modified: self.data.modified.clone(),
                    created: self.data.created.clone()
                };

                ConsoleService::info(&format!("CheckUpdate {:?}", update));


                //FETCHING
                let request = Request::patch("https://atlassian-robot-api.dev-domain.site/robots")
                .header("Content-Type", "application/json")
                .body(Json(&update))
                .expect("Request Error");

                let callback = 
                self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    let status_number = meta.status.as_u16();
                    ConsoleService::info(&format!("Status is{:?}", status_number));
                    if meta.status.is_success(){
                        Msg::Success
                    }else if status_number == 401{
                        Msg::InvalidCredential
                    }else{
                        Msg::Ignore
                    }
                    // else{
                    //     ConsoleService::info(&format!("{:?}", data));
                    //     match data{
                    //         Ok(dataok)=>{
                    //             ConsoleService::info(&format!("Data response {:?}", &dataok));
                    //             Msg::SendData
                    //         }
                    //         Err(Error)=>{
                    //             if status_number == 401{
                    //                 Msg::InvalidCredential
                    //             }else{
                    //             ConsoleService::info("Ignore");
                    //             Msg::Ignore
                    //             }
                    //         }
                    //     }
                    // }
                    
                });

                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                true
            }
            Msg::InputName(data) => {
                ConsoleService::info(&format!("Name : {:?}", data));
                // let test = data.to_owned();
                self.data.name = data;
                true
            }
            Msg::InputDesc(data) => {
                ConsoleService::info(&format!("Description :  {:?}", data));
                // let test = data.to_owned();
                self.data.description = data;
                true
            }
            Msg::InputEmail(data) => {
                ConsoleService::info(&format!("Email : {:?}", data));
                // let test = data.to_owned();
                self.data.platformEmail = data;
                true
            }
            Msg::InputApi(data) => {
                ConsoleService::info(&format!("API : {:?}", data));
                // let test = data.to_owned();
                self.data.platformApiKey = data;
                true
            }
            Msg::InputToken(data) => {
                ConsoleService::info(&format!("Token : {:?}", data));
                // let test = data.to_owned();
                self.data.cloudSessionToken = data;
                true
            }
            // Msg::InputSelect(data) => {
            //     ConsoleService::info(&format!("data input select is {:?}", data));
            //     self.data.platformType = data;
            //     true
            // }
            
            Msg::InputActive(data) => {
                ConsoleService::info(&format!("data input select is {:?}", data));
                self.data.lastActive = data.parse::<i64>().unwrap();
                true
            }
            Msg::InputScheduler(data) => {
                ConsoleService::info(&format!("data input is {:?}", data));
                // let test = data.to_owned();
                self.data.schedule = data.parse::<i64>().unwrap();
                true
            }
            Msg::CheckActiveStatus => {
                // ConsoleService::info("Unchecked");
                // let data = self.checkActiveStatus;
                self.data.checkActiveStatus = !self.data.checkActiveStatus;
                ConsoleService::info(&format!("check active is {:?}", self.data.checkActiveStatus));
                true
            }
            Msg::CheckDoubleEmail => {
                // ConsoleService::info("Unchecked");
                // let data = self.checkActiveStatus;
                self.data.checkDoubleEmail = !self.data.checkDoubleEmail;
                ConsoleService::info(&format!("check double email is {:?}", self.data.checkDoubleEmail));
                true
            }
            Msg::CheckDoubleName => {
                // ConsoleService::info("Unchecked");
                // let data = self.checkActiveStatus;
                self.data.checkDoubleName = !self.data.checkDoubleName;
                ConsoleService::info(&format!("check double name is {:?}", self.data.checkDoubleName));
                true
            }
            Msg::InputText(data) => {
                // let test = data.to_owned();
                ConsoleService::info(&format!("data input is {:?}", data));
                self.username = data;
                true
            }
            Msg::InputSelect(data) => {
                ConsoleService::info(&format!("data input select is {:?}", data));
                self.status = data;
                true
            }

            Msg::RunProgram => {
                self.data.active = !self.data.active;
                ConsoleService::info(&format!("check double name is {:?}", self.data.active));
                true
            }
            
            Msg::GetData(data) => {
                ConsoleService::info(&format!("get data {:?}", data));
                self.project = data;
                // self.router_agent.send(ChangeRoute(AppRoute::Other.into()));
                true
            }
            Msg::FetchOne(data) => {
                self.data = data;
                ConsoleService::info(&format!("get data {:?}", self.data));
                // self.router_agent.send(ChangeRoute(AppRoute::Other.into()));
                true
            }
            Msg::Home =>{
                self.router_agent.send(ChangeRoute(AppRoute::RobotProject.into()));
                true
            }
            Msg::SendData => {
                self.router_agent.send(ChangeRoute(AppRoute::RobotProject.into()));
                true
            }
            Msg::Ignore => {
                false
            }
            Msg::doubleEmailThreshold(data) => {
                self.data.doubleEmailThreshold = data;
                ConsoleService::info(&format!("Email Threshold {:?}", self.data.doubleEmailThreshold));
                true 
            }
            Msg::doubleNameThreshold(data) => {
                self.data.doubleNameThreshold = data;
                ConsoleService::info(&format!("Name Threshold {:?}", self.data.doubleNameThreshold));
                true 
            }
            Msg::CheckSuccess => {
                ConsoleService::info("check success event");
                    self.link.send_message(Msg::UpdateData);
                true
            }
            Msg::InvalidCredential =>{
                self.msg_err.header = "Error".to_string();
                self.msg_err.body = "Invalid Credential".to_string();
                // self.invalidCredential = true;
                true
            }
            Msg::Success => {
                self.msg_err.header = "Success".to_string();
                self.msg_err.body = "Data Updated".to_string();
                // self.link.send_message(Msg::SendData);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render{
            self.link.send_message(Msg::FetchData)
        }
    }

    fn view(&self) -> Html {

        let checked_name=self.data.checkDoubleName;
        let checked_email=self.data.checkDoubleEmail;

        html! {
        <div class="base-form">
            <div class="form">
                
                <h5 style="padding-bottom: 10px">{"Basic Information"}</h5>
                <div class="input-group mb-3" style=" margin: auto; width: 400px;">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Disabled input" 
                        oninput=self.link.callback(|data: InputData| Msg::InputName(data.value))
                        value={self.data.name.clone()}
                        />
                </div>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Description"
                        oninput=self.link.callback(|data: InputData| Msg::InputDesc(data.value))
                        value={self.data.description.clone()}
                        />
                </div>
                <h5>{"Credential Platform"}</h5>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Email"
                        oninput=self.link.callback(|data: InputData| Msg::InputEmail(data.value))
                        value={self.data.platformEmail.clone()}
                        />
                </div>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="API Key"
                        oninput=self.link.callback(|data: InputData| Msg::InputApi(data.value))
                        value={self.data.platformApiKey.clone()}
                        />
                </div>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="API Key"
                        oninput=self.link.callback(|data: InputData| Msg::InputToken(data.value))
                        value={self.data.cloudSessionToken.clone()}
                        />
                </div>
                
                <h5>{"Filter Setting"}</h5>
                <select class="form-select mb-4" style=" margin: auto; width: 400px;" aria-label="Default select example"
                    onchange=self.link.callback(|e| {
                        if let ChangeData::Select(select) = e {
                            let value = select.value();
                            Msg::InputScheduler(value)
                        } else {
                            Msg::InputScheduler("No value".to_string())
                        }
                    })
                >
                    <option>{ self.data.schedule}</option>
                    <option value="3">{ "3 Days" }</option>
                    <option value="7">{ "7 Days" }</option>
                    <option value="14">{ "14 Days" }</option>
                </select>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault" 
                onclick=self.link.callback(|_| Msg::CheckDoubleEmail) checked={self.data.checkDoubleEmail}/>
                <label class="form-check-label" for="flexCheckDefault">{"Double Email"}</label>
                {
                    if checked_email==true{
                        html!{
                            <div style="
                            height: 50px;
                            border-style: ridge">
                                <label for="customRange3" class="form-label">{self.data.doubleEmailThreshold}
                                    <input type="range" class="form-range" min="0" max="100" step="1" id="customRange3"
                                    value={self.data.doubleEmailThreshold.to_string()}
                                    onchange=self.link.callback(|data: ChangeData|{
                                        if let ChangeData::Value(value)=data{
                                            Msg::doubleEmailThreshold(value.parse::<f32>().unwrap())
                                        }else{
                                            Msg::Ignore
                                        }
                                    })
                                    />
                                </label>
                            </div>
                        }
                    }else{
                        html!{

                        }

                    }
                }
                </div>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                onclick=self.link.callback(|_| Msg::CheckDoubleName) checked={self.data.checkDoubleName}/>
                <label class="form-check-label" for="flexCheckDefault">{"Double Name"}</label>
                {
                    if checked_name==true{
                        html!{
                            <div style="
                            height: 50px;
                            border-style: ridge">
                                <label for="customRange3" class="form-label">{self.data.doubleNameThreshold}
                                    <input type="range" class="form-range" min="0" max="100" step="1" id="customRange3"
                                    value={self.data.doubleNameThreshold.to_string()}
                                    onchange=self.link.callback(|data: ChangeData|{
                                        if let ChangeData::Value(value)=data{
                                            Msg::doubleNameThreshold(value.parse::<f32>().unwrap())
                                        }else{
                                            Msg::Ignore
                                        }
                                    })
                                    />
                                </label>
                            </div>
                        }
                    }else{
                        html!{

                        }

                    }
                }
                </div>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                onclick=self.link.callback(|_| Msg::CheckActiveStatus) checked={self.data.checkActiveStatus}/>
                        <label class="form-check-label" for="flexCheckDefault">{"Active Status"}</label>
                </div>
                <select class="form-select mb-4" style=" margin: auto; width: 400px;" aria-label="Default select example"
                    onchange=self.link.callback(|e| {
                        if let ChangeData::Select(select) = e {
                            let value = select.value();
                            Msg::InputActive(value)
                        } else {
                            Msg::InputActive("No value".to_string())
                        }
                    })
                >
                    <option>{self.data.lastActive}</option>
                    <option value="3">{ "3 Days" }</option>
                    <option value="7">{ "7 Days" }</option>
                    <option value="14">{ "14 Days" }</option>
                </select>
                <div style="display:flex; justify-content: space-around;">
                {
                    if self.data.active{
                        html!{
                        <button type="button" class="btn btn-secondary"
                        onclick=self.link.callback(|_|Msg::RunProgram)
                        >{"Stop"}</button>
                        }
                    } 
                    else{
                        html!{
                        <button type="button" class="btn btn-primary"
                        onclick=self.link.callback(|_|Msg::RunProgram)
                        >{"Start"}</button>
                        }
                    }
                }     
                <button type="button" class="btn btn-danger" data-bs-toggle="modal" data-bs-target="#exampleModal"
                >{"Delete"}</button>
                <button type="button" class="btn btn-success" data-bs-toggle="modal" data-bs-target="#savedModal"
                >{"Save"}
                
                </button>
                </div>
                </div>
                


                <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"Delete Project Confirmation"}</h5>
                                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body">
                                
                                {&format!("Are you sure want to Delete bot with name {:?}?",self.data.name.clone())}
                                
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                                <button type="button" class="btn btn-danger" data-bs-dismiss="modal"
                                onclick = self.link.callback(|_|Msg::DeleteData)>{"Delete"}</button>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="modal fade" id="savedModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"Save Changes"}</h5>
                                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body">
                                {"Have you make sure due to field changed?"}
                            </div>
                                <div class="modal-footer">
                                    <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                                    <button type="button" class="btn btn-primary" data-bs-dismiss="modal" data-bs-toggle="modal" data-bs-target="#Invalid"
                                    onclick=self.link.callback(|_|Msg::CheckSuccess)>{"Save changes"}
                                    </button>  
                                </div>
                        </div>
                    </div>
                </div> 
                {self.msg_2()}
            </div>
        }
    }
}

impl RobotInput{
    fn msg_2(&self)->Html{
        ConsoleService::info("test modal");
        html!{
            <div style="background: #A73034; font-family: Alexandria; color: #A73034;" >
                <div class="modal fade" id="Invalid" data-bs-backdrop="static" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="false"
                >
                    <div class="modal-dialog"
                    >
                        <div class="modal-content"
                        >
                            <div class="modal-header"
                            >
                                <h5 class="modal-tittle"> <p> {format!("{}!",self.msg_err.header)} </p> </h5>
                                <button 
                                    type="button"
                                    class="btn-close"
                                    data-bs-dismiss="modal"
                                    aria-label="close"
                                    onclick=self.link.callback(|_|Msg::Ignore)
                                >
                                </button>
                            </div>
                            <div class="modal-body" style="color:black;" >
                                <p> {format!("{} !",self.msg_err.body)} </p>
                            </div>
                            <div class="modal-footer"
                            >
                                <button
                                    type="button"
                                    style="
                                        background:#A73034;
                                        border-color:#A73034;
                                        color:white;
                                        border-radius:15px;
                                        width: 70px;
                                        height: 40px; 
                                    "

                                    class="btn btn-primary"
                                    data-bs-dismiss="modal"
                                    onclick={
                                        if self.msg_err.header.eq("Error"){
                                            self.link.callback(|_| Msg::Ignore)
                                        }else{
                                            self.link.callback(|_| Msg::SendData)
                                        }
                                    }
                                >
                                <p> {"Close"} </p>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
        
    }
}

//DRAFT
// <select class="form-select mb-4" style=" margin: auto; width: 400px;" aria-label="Default select example"
                //     onchange=self.link.callback(|e| {
                //         if let ChangeData::Select(select) = e {
                //             let value = select.value();
                //             Msg::InputSelect(value)
                //         } else {
                //             Msg::InputSelect("No value".to_string())
                //         }
                //     })
                // >
                //     <option>{self.data.platformType.clone()}</option>
                //     <option value="SERVER">{ "Server" }</option>
                //     <option value="CLOUD">{ "Cloud" }</option>
                // </select>
                 // onchange=self.link.callback(|_| {
                //     // Msg::SelectProject
                // })
                    // <div>
                    //     <button type="button" class="btn btn-primary mb-3" data-bs-toggle="modal" data-bs-target="#exampleModal">
                    //         {"Project Selection"}
                    //     </button>
                    // </div>
                    //push