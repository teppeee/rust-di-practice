

fn main() {
    let state = AppState{
        connection: Con,
    };

    use_service_a(state);
}

pub fn use_service_a(svc : impl ServiceA){
    println!("{}", svc.a_method());
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
        let repoa = a.get_user();

        let b = self.get_repo_b();
        let repob =b.get_customer();

        let result = format!("{}:{}:service_a_method", repoa, repob);

        result
    }
}

//通常の実装　すべてAppStateに実装してやる
impl ServiceA for AppState {}

impl HaveRepositoryA for AppState {
    type RepoA = Self;

    fn get_repo_a(&self) -> &Self::RepoA {
        &self
    }
}

impl HaveRepositoryB for AppState {
    type RepoB = Self;

    fn get_repo_b(&self) -> &Self::RepoB {
        &self
    }
}

impl HaveServiceA for AppState {
    type SvcA = Self;

    fn get_service_a(&self) -> &Self::SvcA {
        &self
    }
}


//テスト
//ServiceAを差し替えたい
struct Mock{}

impl ExtServiceA for Mock {
    fn a_method(&self) -> String {
        "service_a_mock".to_string()
    }
}

#[test]
fn test_use_service_a(){
    let mock = Mock{};
    assert_eq!("service_a_mock".to_string(), mock.a_method()) 
}


//リポジトリがまだなのでそこを差し替えでServiceAをテストしたい
struct MockRepo{}

impl ExtRepositryA for MockRepo {
    fn get_user(&self) -> String {
        "mock_repo_a".to_string()
    }
}

impl ExtRepositryB for MockRepo {
    fn get_customer(&self) -> String {
        "mock_repo_b".to_string()
    }
}

impl ServiceA for MockRepo {}

impl HaveRepositoryA for MockRepo {
    type RepoA = Self;

    fn get_repo_a(&self) -> &Self::RepoA {
        &self
    }
}

impl HaveRepositoryB for MockRepo {
    type RepoB = Self;

    fn get_repo_b(&self) -> &Self::RepoB {
        &self
    }
}

#[test]
fn test_use_service_a_repo(){
    let mock = MockRepo{};
    assert_eq!("mock_repo_a:mock_repo_b:service_a_method".to_string(), mock.a_method()) 
}

