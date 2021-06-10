fn main() {
    println!("Hello, world!");
}

struct Con;

struct AppState {
    connection: Con,
}

// pub trait HaveCon {
//     type Con;
//     fn get_con(&self) -> &Self::Con;
// }

pub trait HaveRepositoryA {
    type RepoA: ExtRepositryA;
    fn get_repo_a(&self) -> &Self::RepoA;
}

pub trait RepositoryA /* : HaveCon*/ {}

pub trait ExtRepositryA {
    fn get_user(&self) -> String;
}

impl ExtRepositryA for AppState {
    fn get_user(&self) -> String {
        //exceute query by con
        &self.connection;
        "extrepo_a".to_string()
    }
}

pub trait HaveServiceA {
    type SvcA: ServiceA;
    fn get_service_a(&self) -> &Self::SvcA;
}


pub trait HaveRepositoryB {
    type RepoB: ExtRepositryB;
    fn get_repo_b(&self) -> &Self::RepoB;
}


pub trait RepositoryB{}

pub trait ExtRepositryB{
    fn get_customer(&self) -> String;
}

impl ExtRepositryB for AppState {
    fn get_customer(&self) -> String {
        &self.connection;
        "extrepo_b".to_string()
    }
}

pub trait HaveRepositryB {}

pub trait ServiceA : HaveRepositoryA + HaveRepositoryB {}

pub trait ExtServiceA{
    fn a_method(&self) -> String;
}

impl<T: ServiceA> ExtServiceA for T {
    fn a_method(&self) -> String {
        let a = self.get_repo_a();
        //メイン実装
        a.get_user();

        let b = self.get_repo_b();
        b.get_customer();

        "".to_string()
    }
}
