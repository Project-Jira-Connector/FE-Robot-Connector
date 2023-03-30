use yew_router::prelude::*;



#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to="/LandingPage"]
    LandingPage,
    #[to="/ConnectorCreate"]
    ConnectorCreate,
    #[to="/ConnectorSetting/{_name}"]
    ConnectorSetting { _name: String },
    #[to="/ConnectorHome"]
    ConnectorHome,
    #[to="/RobotHome"]
    RobotHome,
    #[to="/RobotCreate"]
    RobotCreate,
    #[to="/RobotInput/{idProject}"]
    RobotInput{idProject : String},
    #[to="/RobotProject"]
    RobotProject,
    #[to="/"]
    Home,
}