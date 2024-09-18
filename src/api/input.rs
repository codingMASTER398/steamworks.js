use napi_derive::napi;
use steamworks_sys::EInputActionOrigin;

#[napi]
pub mod input {
    use napi::bindgen_prelude::BigInt;

    #[napi(string_enum)]
    pub enum InputType {
        Unknown,
        SteamController,
        XBox360Controller,
        XBoxOneController,
        GenericGamepad,
        PS4Controller,
        AppleMFiController,
        AndroidController,
        SwitchJoyConPair,
        SwitchJoyConSingle,
        SwitchProController,
        MobileTouch,
        PS3Controller,
        PS5Controller,
        SteamDeckController,
    }

    impl From<steamworks::InputType> for InputType {
        fn from(input_type: steamworks::InputType) -> InputType {
            match input_type {
                steamworks::InputType::Unknown => InputType::Unknown,
                steamworks::InputType::SteamController => InputType::SteamController,
                steamworks::InputType::XBox360Controller => InputType::XBox360Controller,
                steamworks::InputType::XBoxOneController => InputType::XBoxOneController,
                steamworks::InputType::GenericGamepad => InputType::GenericGamepad,
                steamworks::InputType::PS4Controller => InputType::PS4Controller,
                steamworks::InputType::AppleMFiController => InputType::AppleMFiController,
                steamworks::InputType::AndroidController => InputType::AndroidController,
                steamworks::InputType::SwitchJoyConPair => InputType::SwitchJoyConPair,
                steamworks::InputType::SwitchJoyConSingle => InputType::SwitchJoyConSingle,
                steamworks::InputType::SwitchProController => InputType::SwitchProController,
                steamworks::InputType::MobileTouch => InputType::MobileTouch,
                steamworks::InputType::PS3Controller => InputType::PS3Controller,
                steamworks::InputType::PS5Controller => InputType::PS5Controller,
                steamworks::InputType::SteamDeckController => InputType::SteamDeckController,
            }
        }
    }

    #[napi]
    pub struct Controller {
        pub(crate) handle: BigInt,
    }

    #[napi]
    impl Controller {
        #[napi]
        pub fn activate_action_set(&self, action_set_handle: BigInt) {
            let client = crate::client::get_client();
            client
                .input()
                .activate_action_set_handle(self.handle.get_u64().1, action_set_handle.get_u64().1)
        }

        #[napi]
        pub fn is_digital_action_pressed(&self, action_handle: BigInt) -> bool {
            let client = crate::client::get_client();
            client
                .input()
                .get_digital_action_data(self.handle.get_u64().1, action_handle.get_u64().1)
                .bState
        }

        #[napi]
        pub fn get_analog_action_vector(&self, action_handle: BigInt) -> AnalogActionVector {
            let client = crate::client::get_client();
            let data = client
                .input()
                .get_analog_action_data(self.handle.get_u64().1, action_handle.get_u64().1);
            AnalogActionVector {
                x: data.x as f64,
                y: data.y as f64,
            }
        }

        #[napi]
        pub fn get_type(&self) -> InputType {
            let client = crate::client::get_client();
            client
                .input()
                .get_input_type_for_handle(self.handle.get_u64().1)
                .into()
        }

        #[napi]
        pub fn get_handle(&self) -> BigInt {
            self.handle.clone()
        }

        #[napi]
        pub fn show_binding_panel(&self) -> bool {
            let client = crate::client::get_client();
            client.input().show_binding_panel(self.handle.get_u64().1)
        } 

        #[napi]
        pub fn get_digital_action_origins(
            &self,
            action_set_handle: BigInt,
            digital_action_handle: BigInt,
        ) -> Vec<BigInt> {
            let client = crate::client::get_client();
            let origins = client.input().get_digital_action_origins(
                self.handle.get_u64().1,
                action_set_handle.get_u64().1,
                digital_action_handle.get_u64().1,
            );
    
            origins
                .iter()
                .map(|&origin| BigInt::from(origin as i64)) // Cast to BigInt
                .collect()
        }
    
        /// Get analog action origins for the specified action and action set.
        #[napi]
        pub fn get_analog_action_origins(
            &self,
            action_set_handle: BigInt,
            analog_action_handle: BigInt,
        ) -> Vec<BigInt> {
            let client = crate::client::get_client();
            let origins = client.input().get_analog_action_origins(
                self.handle.get_u64().1,
                action_set_handle.get_u64().1,
                analog_action_handle.get_u64().1,
            );
    
            origins
                .iter()
                .map(|&origin| BigInt::from(origin as i64)) // Cast to BigInt
                .collect()
        }
    }

    #[napi(object)]
    pub struct AnalogActionVector {
        pub x: f64,
        pub y: f64,
    }

    #[napi]
    pub fn init() {
        let client = crate::client::get_client();
        client.input().init(false);
    }

    #[napi]
    pub fn get_controllers() -> Vec<Controller> {
        let client = crate::client::get_client();
        client
            .input()
            .get_connected_controllers()
            .into_iter()
            .map(|identity| Controller {
                handle: BigInt::from(identity),
            })
            .collect()
    }

    #[napi]
    pub fn get_action_set(action_set_name: String) -> BigInt {
        let client = crate::client::get_client();
        BigInt::from(client.input().get_action_set_handle(&action_set_name))
    }

    #[napi]
    pub fn get_digital_action(action_name: String) -> BigInt {
        let client = crate::client::get_client();
        BigInt::from(client.input().get_digital_action_handle(&action_name))
    }

    #[napi]
    pub fn get_analog_action(action_name: String) -> BigInt {
        let client = crate::client::get_client();
        BigInt::from(client.input().get_analog_action_handle(&action_name))
    }

    #[napi]
    pub fn shutdown() {
        let client = crate::client::get_client();
        client.input().shutdown()
    }
    #[napi]
    pub fn run_frame() {
        let client = crate::client::get_client();
        client.input().run_frame();
    }

    #[napi]
    pub fn get_glyph_for_action_origin(origin: BigInt) -> String {
        let client = crate::client::get_client();
        let origin_value = origin.get_u64().1 as steamworks_sys::EInputActionOrigin; // Convert BigInt to EInputActionOrigin
        client.input().get_glyph_for_action_origin(origin_value)
    }
}
