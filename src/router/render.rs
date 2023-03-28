use yew::prelude::*;
use yew_router::{
    prelude::*,
    service::RouteService,
};

use crate::pages::{
    ConnectorHome::ConnectorHome,
    ConnectorCreate::ConnectorCreate,
    ConnectorSetting::ConnectorSetting,
    RobotCreate::RobotCreate,
    RobotInput::RobotInput,
    RobotProject::RobotProject,
    RobotHome::RobotHome,
};
use crate::router::route::AppRoute;

pub enum Msg {}


pub struct Render {}

impl Component for Render {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: AppRoute| {
            let mut route_service = RouteService::new();
            match switch {
                // Connector
                AppRoute::ConnectorHome => {
                    html! {
                        <ConnectorHome/>
                    }
                }
                AppRoute::ConnectorCreate => {
                    html! {
                        <ConnectorCreate/>
                    }
                }
                AppRoute::ConnectorSetting {_name} => {
                    html! {
                        <ConnectorSetting _name=_name/>
                    }
                }
                // Robot
                AppRoute::RobotInput {idProject} => {
                    html! {
                        <RobotInput idProject=idProject/>
                    }
                }
                AppRoute::RobotCreate => {
                    html! {
                        <RobotCreate/>
                    }
                }
                AppRoute::RobotHome => {
                    html! {
                        <RobotHome/>
                    }
                }
                AppRoute::RobotProject => {
                    html! {
                        <RobotProject/>
                    }
                }
                _ => {
                    route_service.set_route("/", ());
                    html! {
                        <ConnectorHome/>
                    }

                }
            }
        });


        html! {
            <Router<AppRoute, ()> render=render/>
        }
    }
}
