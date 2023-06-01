#[cfg(test)]
mod tests 
    {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;
    #[test]
    fn pid_init() 
        {
        let pid = PID::new();
        assert_eq!(pid.Kp, 0.0);
        }

    #[test]
    fn update_test()
        {
        let pid = PID::new();
        pid.update(1.0, 0.5, 0.2);
        }
    }
