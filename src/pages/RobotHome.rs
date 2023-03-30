use yew::prelude::*;

// use crate::pages::content::Content;

use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};

use yew_router::prelude::*;
use crate::router::route::AppRoute;


pub enum Msg {
    AddOne,
    InputText(String),
}

pub struct PageInput {

    username: String,
    status: String,
}

pub struct RobotHome {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    message: String,
}

impl Component for RobotHome {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Homepage");
        Self {
            link,
            value: 0,
            message: String::from("Initial Message"),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
            //     input.focus();
            // }

            ConsoleService::info("First Render");
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::InputText(data) => {
                self.message = data;
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

    fn view(&self) -> Html {

        type Anchor = RouterAnchor<AppRoute>;

        html! {
            <div class="base">
                <div class="homepage">
                    <div class="container-md" style="justify-content: flex-start;
                    display: flex;
                    border-radius: 10px;
                    position: absolute;
                    right: 0;
                    left: 0;
                    margin-top: 100px;
                    width: 100%;">
                        <div>
                            <img src="img/robot.png" alt="rust-image" width="500" height="500"
                            style="
                            margin: 20px"/>   
                        </div>
                        <div style="">
                            <h4 style="padding-top: 10px; 
                            font-size: 40px;
                            color: #A3C7D6; ">{"welcome to"}</h4>
                            <h3 style=
                            "font-weight: bold;
                            font-size: 65px;
                            color: #2E4F4F;
                            line-height: 0.9;">{"Atlassian Robot Automation"}</h3>
                            <p style=
                            "padding-top: 20px;
                            padding-right: 10px">
                                    {"Robotic process automation (RPA)
                                    is a software technology that makes
                                    it easy to build, deploy, and manage
                                    software robots that emulate humans actions
                                    interacting with digital systems and software.
                                    With automation robots built on this platform, 
                                    it is intended to simplify your task management. 
                                    Go explore more, be wild, and be more productive!
                                    "}
                                    </p>
                                <div style=" text-decoration: none; color: rgb(100,100,100); padding-top: 20px">
                                    <Anchor route=AppRoute::RobotCreate>
                                        <label class="link btn btn-secondary" style=" text-decoration: none!important;" >
                                            {"Create New"}
                                        </label>
                                    </Anchor>
                                </div>
                        </div>
                    </div>
                    </div>
                </div>
        }
    }
}
//push
