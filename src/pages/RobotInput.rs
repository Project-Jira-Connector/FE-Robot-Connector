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
    UserAccount,
    UsersData,
    ProjectId,
    PostReturnValue,
    Users
};

use crate::pages::RobotCreate::{
    RobotCreate
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
                platformType: String::from(""),
                cloudSessionToken: String::from(""),
                active: false,
                schedule: 0,
                lastActive: 0,
                checkDoubleName: false,
                checkDoubleEmail: false,
                checkActiveStatus: false
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
                    self.link.callback(|response: Response<Json<Result<Vec<UsersData>, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();

                        let status_number = meta.status.as_u16();

                        ConsoleService::info(&format!("status is {:?}", status_number));

                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::FetchOne(dataok.get(0).unwrap().clone())
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
                    platformType: self.data.platformType.clone(),
                    cloudSessionToken: self.data.cloudSessionToken.clone(),
                    active: self.data.active.clone(),
                    schedule: self.data.schedule.clone(),
                    lastActive: self.data.lastActive.clone(),
                    checkActiveStatus: self.data.checkActiveStatus.clone(),
                    checkDoubleName: self.data.checkDoubleName.clone(),
                    checkDoubleEmail: self.data.checkDoubleEmail.clone(),
                };

                ConsoleService::info(&format!("CheckUpdate {:?}", update));


                //FETCHING
                let request = Request::patch("https://atlassian-robot-api.dev-domain.site/robots")
                .header("Content-Type", "application/json")
                .body(Json(&update))
                .expect("Request Error");

                let callback = 
                self.link.callback(|response: Response<Json<Result<PostReturnValue, anyhow::Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    let status_number = meta.status.as_u16();
                    ConsoleService::info(&format!("Status is{:?}", status_number));
                    if meta.status.is_success(){
                        Msg::SendData
                    }
                    else{
                        match data{
                            Ok(dataok)=>{
                                ConsoleService::info(&format!("Data response {:?}", &dataok));
                                Msg::SendData
                            }
                            Err(error)=>{
                                ConsoleService::info("Ignore");
                                Msg::Ignore
                            }
                        }
                    }
                    
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
            Msg::InputSelect(data) => {
                ConsoleService::info(&format!("data input select is {:?}", data));
                self.data.platformType = data;
                true
            }
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



        html! {
        <div class="base-form">
            <div class="form">
                <div class="img-setting">
                    <img src="img/pngegg.png" alt="rust image" width="200" height="200"/>
                </div>
                <h5>{"Basic Information"}</h5>
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
                <select class="form-select mb-4" style=" margin: auto; width: 400px;" aria-label="Default select example"
                    onchange=self.link.callback(|e| {
                        if let ChangeData::Select(select) = e {
                            let value = select.value();
                            Msg::InputSelect(value)
                        } else {
                            Msg::InputSelect("No value".to_string())
                        }
                    })
                >
                    <option>{self.data.platformType.clone()}</option>
                    <option value="SERVER">{ "Server" }</option>
                    <option value="CLOUD">{ "Cloud" }</option>
                </select>
                 // onchange=self.link.callback(|_| {
                //     // Msg::SelectProject
                // })
                    // <div>
                    //     <button type="button" class="btn btn-primary mb-3" data-bs-toggle="modal" data-bs-target="#exampleModal">
                    //         {"Project Selection"}
                    //     </button>
                    // </div>
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
                </div>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault"
                onclick=self.link.callback(|_| Msg::CheckDoubleName) checked={self.data.checkDoubleName}/>
                        <label class="form-check-label" for="flexCheckDefault">{"Double Name"}</label>
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
                <button type="button" class="btn btn-success"
                onclick=self.link.callback(|_|Msg::UpdateData)>{"Patch"}
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
                                
                                
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                                <button type="button" class="btn btn-danger" data-bs-dismiss="modal"
                                onclick = self.link.callback(|_|Msg::DeleteData)>{"Delete"}</button>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"Modal title"}</h5>
                                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body">
                            </div>
                                <div class="modal-footer">
                                    <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                                    <button type="button" class="btn btn-primary">{"Save changes"}</button>
                                </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
