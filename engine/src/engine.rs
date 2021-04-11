mod vesta {
    struct Engine {
        config: Config
    }
    
    impl Engine {
        pub fn new(config: Config) -> Self {
            
            Engine {
                config
            }
        }
    }
}