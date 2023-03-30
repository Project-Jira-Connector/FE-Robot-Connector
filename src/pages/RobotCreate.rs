use anyhow::Error;
use yew::{prelude::*, callback};

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

use crate::router::route::AppRoute;
use crate::types::var::{
    Users
};

pub enum Msg {
    RequestPostData,
    // InputText(String),
    InputName(String),
    InputDesc(String),
    InputEmail(String),
    InputScheduler(String),
    InputApi(String),
    InputToken(String),
    InputSelect(String),
    InputActive(String),
    doubleEmailThreshold(f32),
    doubleNameThreshold(f32),
    CheckActiveStatus,
    CheckDoubleName,
    CheckDoubleEmail,
    GetData(String),
    Ignore,
    GetName(Users),
}


pub struct RobotCreate {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    // DATA
    name: String,
    description: String,
    platformEmail: String,
    platformApiKey: String,
    platformType: String,
    cloudSessionToken: String,
    schedule: i64,
    lastActive: i64,
    active: bool,
    checkActiveStatus: bool,
    checkDoubleName: bool,
    checkDoubleEmail: bool,
    doubleNameThreshold: f32,
    doubleEmailThreshold: f32,
    
    username: String,
    status: String,

    // SERVICES
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

impl Component for RobotCreate {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            // DATA
            name: String::from(""),
            description: String::from(""),
            platformApiKey: String::from(""),
            platformEmail: String::from(""),
            cloudSessionToken: String::from(""),
            platformType: String::from(""),
            schedule: 0,
            lastActive: 0,
            active: false,
            checkActiveStatus: false,
            checkDoubleName: false,
            checkDoubleEmail: false,
            doubleNameThreshold: 100.0,
            doubleEmailThreshold: 100.0,

            username: String::from(""),
            status: String::from(""),

            // SERVICES
            link: link.clone(),
            fetch_task: None,
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

            Msg::GetName(data) => {
                true
            }

            Msg::RequestPostData => {

                let user = Users {
                    name: self.name.clone(),
                    description: self.description.clone(),
                    platformEmail: self.platformEmail.clone(),
                    platformApiKey: self.platformApiKey.clone(),
                    // platformType: self.platformType.clone(),
                    cloudSessionToken: self.cloudSessionToken.clone(),
                    active: self.active.clone(),
                    schedule: self.schedule.clone(),
                    lastActive: self.lastActive.clone(),
                    checkActiveStatus: self.checkActiveStatus.clone(),
                    checkDoubleName: self.checkDoubleName.clone(),
                    checkDoubleEmail: self.checkDoubleEmail.clone(),
                    doubleEmailThreshold: self.doubleEmailThreshold.clone(),
                    doubleNameThreshold: self.doubleNameThreshold.clone()
                };

                //FETCHING
                let request = Request::post("https://atlassian-robot-api.dev-domain.site/robots")
                .header("Content-Type", "application/json")
                .body(Json(&user))
                .expect("Request Error");

                let callback = 
                self.link.callback(|response: Response<Json<Result<Users, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    let status_number = meta.status.as_u16();
                    ConsoleService::info(&format!("Status is{:?}", status_number));
                    match data{
                        Ok(dataok)=>{
                            ConsoleService::info(&format!("Data response {:?}", &dataok));
                            Msg::GetData(format!("{:?}", dataok))
                        }
                        Err(error)=>{
                            ConsoleService::info("Ignore");
                            ConsoleService::info(&format!("Data error {:?}", error));

                            Msg::Ignore
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
                self.name = data;
                true
            }
            Msg::InputDesc(data) => {
                ConsoleService::info(&format!("Description :  {:?}", data));
                // let test = data.to_owned();
                self.description = data;
                true
            }
            Msg::InputEmail(data) => {
                ConsoleService::info(&format!("Email : {:?}", data));
                // let test = data.to_owned();
                self.platformEmail = data;
                true
            }
            Msg::InputApi(data) => {
                ConsoleService::info(&format!("API : {:?}", data));
                // let test = data.to_owned();
                self.platformApiKey = data;
                true
            }
            Msg::InputToken(data) => {
                ConsoleService::info(&format!("Token : {:?}", data));
                // let test = data.to_owned();
                self.cloudSessionToken = data;
                true
            }
            Msg::InputScheduler(data) => {
                ConsoleService::info(&format!("data input is {:?}", data));
                // let test = data.to_owned();
                self.schedule = data.parse::<i64>().unwrap();
                true
            }
            Msg::InputSelect(data) => {
                ConsoleService::info(&format!("data input select is {:?}", data));
                self.platformType = data;
                true
            }
            Msg::InputActive(data) => {
                ConsoleService::info(&format!("data input select is {:?}", data));
                self.lastActive = data.parse::<i64>().unwrap();
                true
            }
            Msg::CheckDoubleEmail => {
                // ConsoleService::info("Unchecked");
                // let data = self.checkActiveStatus;
                self.checkDoubleEmail = !self.checkDoubleEmail;
                ConsoleService::info(&format!("check double email is {:?}", self.checkDoubleEmail));
                true
            }
            Msg::CheckDoubleName => {
                // ConsoleService::info("Unchecked");
                // let data = self.checkActiveStatus;
                self.checkDoubleName = !self.checkDoubleName;
                ConsoleService::info(&format!("check double name is {:?}", self.checkDoubleName));
                true
            }
            Msg::CheckActiveStatus => {
                // ConsoleService::info("Unchecked");
                // let data = self.checkActiveStatus;
                self.checkActiveStatus = !self.checkActiveStatus;
                ConsoleService::info(&format!("check double name is {:?}", self.checkActiveStatus));
                true
            }
            Msg::doubleNameThreshold(data)=> {
                self.doubleNameThreshold = data;
                ConsoleService::info(&format!("Name Threshold {:?}", self.doubleNameThreshold));
                true
            }
            Msg::doubleEmailThreshold(data) => {
                self.doubleEmailThreshold = data;
                ConsoleService::info(&format!("Email Threshold {:?}", self.doubleEmailThreshold));
                true 
            }

            Msg::GetData(data) => {
                ConsoleService::info(&format!("get data {:?}", data));

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

    fn view(&self) -> Html {

        let checked_email=self.checkDoubleEmail;
        let checked_name=self.checkDoubleName;

        // let onchange=self.link.batch_callback(|data|{
        //     if let ChangeData::Value(value) = data{
        //         Some(value.parse::<f32>().unwrap())
        //     }else{
        //       None
        //     }
        // });

        html! {
            <div class="base-form">
                <div class="create">
                <h5>{"Basic Information"}</h5>
                <div class="input-group mb-3" style=" margin: auto; width: 400px;">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Name" 
                        oninput=self.link.callback(|data: InputData| Msg::InputName(data.value))
                        />
                </div>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Description"
                        oninput=self.link.callback(|data: InputData| Msg::InputDesc(data.value))
                        />
                </div>
                <h5>{"Credential Platform"}</h5>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="API Key" required=true
                        oninput=self.link.callback(|data: InputData| Msg::InputApi(data.value))
                        />
                </div>
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="Email (your_email@gmail.com)"
                        oninput=self.link.callback(|data: InputData| Msg::InputEmail(data.value))
                        />
                </div>                
                <div class="input-group mb-3" style=" margin: auto; width: 400px">
                    <span class="input-group-text"></span>
                        <input type="text" class="form-control" placeholder="TokenID"
                        oninput=self.link.callback(|data: InputData| Msg::InputToken(data.value))
                        />
                </div>
                
                // <select class="form-select mb-3" style=" margin: auto; width: 400px;" aria-label="Default select example"
                //     onchange=self.link.callback(|e| {
                //         if let ChangeData::Select(select) = e {
                //             let value = select.value();
                //             Msg::InputSelect(value)
                //         } else {
                //             Msg::InputSelect("No value".to_string())
                //         }
                //     })
                // >
                //     <option>{ "Select Platform"}</option>
                //     <option value="CLOUD">{ "Cloud" }</option>
                //     <option value="SERVER">{ "Server" }</option>
                // </select>
                // <h5>{"Notification Setting"}</h5>
                // <div class="input-group mb-3" style=" margin: auto; width: 400px">
                //     <span class="input-group-text"></span>
                //         <input type="text" class="form-control" placeholder="Email (your_email@gmail.com)"
                //         oninput=self.link.callback(|data: InputData| Msg::InputText(data.value))
                //         />
                // </div>
                // <div class="input-group mb-3" style=" margin: auto; width: 400px">
                //     <span class="input-group-text"></span>
                //         <input type="text" class="form-control" placeholder="Password"
                //         oninput=self.link.callback(|data: InputData| Msg::InputText(data.value))
                //         />
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
                    <option>{ "Scheduler"}</option>
                    <option value="3">{ "3 days" }</option>
                    <option value="7">{ "7 days" }</option>
                    <option value="14">{ "14 days" }</option>
                </select>
                
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault" onclick=self.link.callback(|_| Msg::CheckDoubleEmail) checked={self.checkDoubleEmail}/>
                {
                    if checked_email==true{
                        html!{
                            <label for="customRange3" class="form-label">{"Accuracy 0-100"}
                                <input type="range" class="form-range" min="0" max="100" step="1" id="customRange3"
                                value={self.doubleEmailThreshold.to_string()}
                                onchange=self.link.callback(|data: ChangeData|{
                                    if let ChangeData::Value(value)=data{
                                        Msg::doubleEmailThreshold(value.parse::<f32>().unwrap())
                                    }else{
                                        Msg::Ignore
                                    }
                                })
                                />
                            </label>
                        }
                    }else{
                        html!{

                        }

                    }
                }
                        <label class="form-check-label" for="flexCheckDefault">{"Double Email"}</label>
                </div>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault" onclick=self.link.callback(|_| Msg::CheckDoubleName) checked={self.checkDoubleName}/>
                {
                    if checked_name==true{
                        html!{
                            <label for="customRange3" class="form-label">{"Accuracy 0-100"}
                                <input type="range" class="form-range" min="0" max="100" step="1" id="customRange3"
                                value={self.doubleNameThreshold.to_string()}
                                onchange=self.link.callback(|data: ChangeData|{
                                    if let ChangeData::Value(value)=data{
                                        Msg::doubleNameThreshold(value.parse::<f32>().unwrap())
                                    }else{
                                        Msg::Ignore
                                    }
                                })
                                />
                            </label>
                        }
                    }else{
                        html!{

                        }

                    }
                }
                    <label class="form-check-label" for="flexCheckDefault">{"Double Name"}</label>
                </div>
                <div class="form-check mb-3" style="margin: auto; width:400px;">
                <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault" onclick=self.link.callback(|_| Msg::CheckActiveStatus) checked={self.checkActiveStatus}/> 
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
                    <option>{ "Last Active"}</option>
                    <option value="3">{ "3 days" }</option>
                    <option value="7">{ "7 days" }</option>
                    <option value="14">{ "14 days" }</option>
                </select>

                <button
                    type="button"
                    class="btn btn-primary"
                    onclick=self.link.callback(|_| {
                        Msg::RequestPostData
                    })
                >       
                    { "Create" }
                </button>
                </div>
            </div>
        }
    }
}
