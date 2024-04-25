
use serde::{Deserialize, Serialize};

#[derive(Clone,Debug,serde::Serialize, serde::Deserialize)]
pub struct Enumeration {
    pub id: String,
    pub server_id: i32,
    pub rejection_id: i32,
    pub sync_status: i32,
    pub rejection_reason: String,
    pub date_submitted: Option<String>,

    pub token: String,
    pub area_office: Option<String>,
    pub service_center: Option<String>,
    pub enumeration_type: Option<String>,
    pub dt_name_number: Option<String>,
    pub feeder_name: Option<String>,

    pub unique_dt_number_photo: Option<String>,
    pub unique_dt_number_photo_gps: Option<String>,
    pub unique_dt_number: Option<String>,
    pub dt_location_gps: Option<String>,

    pub upriser_number: i32,
    pub street_address: Option<String>,
    pub pole_number: Option<String>,
    pub pole_gps: Option<String>,
    pub premise_name: Option<String>,
    pub premise_location_gps: Option<String>,
    pub closest_landmark: Option<String>,
    pub premise_category: Option<String>,
    pub type_of_premise: Option<String>,
    pub is_enumeration_possible_premise: bool,
    pub building_name_number: Option<String>,
    pub building_location_gps: Option<String>,
    pub is_enumeration_possible_building: bool,
    pub is_enumeration_possible_customer: bool,
    pub customer_id_type: Option<String>,
    pub customer_id: Option<String>,
    pub occupant_role: Option<String>,
    pub surname: Option<String>,
    pub name: Option<String>,
    pub number_of_occupants: Option<String>,
    pub mobile_phone: Option<String>,
    pub landline: Option<String>,
    pub email: Option<String>,
    pub is_generator_present: bool,
    pub generator_kva: f32,
    pub is_aedc_customer: bool,
    pub occupant_payment_method: Option<String>,
    pub general_notes: Option<String>,
    pub connection_location_gps: Option<String>,
    pub is_account_details_avail: bool,
    pub is_connection_accessible: bool,

    pub first_or_second_notice: Option<String>,
    pub unique_number_first_notice: Option<String>,
    pub unique_number_second_notice: Option<String>,
    pub type_of_notice: Option<String>,
    pub notice_comment: Option<String>,

    pub photo_of_pole: Option<String>,
    pub photo_of_premise: Option<String>,
    pub photo_of_notice: Option<String>,

    pub connection_type_of_connection: Option<String>,
    pub connection_unit_name: Option<String>,
    pub connection_floor: Option<String>,
    pub connection_photo: Option<String>,
    pub connection_photo_gps: Option<String>,
    pub is_sub_connection: bool,
    pub number_of_phases: Option<String>,
    pub number_of_phases_connected: Option<String>,
    pub connection_billing_type: Option<String>,
    pub is_technical_follow_up_needed: bool,
    pub technical_follow_up_reason: Option<String>,
    pub technical_follow_up_comment: Option<String>,
    pub is_power_supply_on_at_enumeration: bool,
    pub is_shared_account: bool,
    pub account_name: Option<String>,
    pub account_number: Option<String>,
    pub new_account_number: Option<String>,
    pub contract_number: Option<String>,
    pub tariff_class: Option<String>,
    pub recommended_tariff_class: Option<String>,
    pub photo_of_account: Option<String>,
    pub account_gps: Option<String>,
    pub unique_enumeration_id: i32,
    pub unique_enumeration_photo: Option<String>,

    pub meter_photo: Option<String>,
    pub meter_location_gps: Option<String>,
    pub second_meter_photo: Option<String>,
    pub second_meter_location_gps: Option<String>,
    pub is_meter_sealed: bool,
    pub seal_number1: Option<String>,
    pub meter_billing_type: Option<String>,
    pub meter_manufacturer: Option<String>,
    pub meter_manufacturer_type: Option<String>,
    pub is_meter_connected: bool,
    pub meter_condition: Option<String>,
    pub meter_number: Option<String>,
    pub current_consumption_credits: Option<String>,
    pub faulty_installation: Option<String>,
    pub faulty_comment: Option<String>,

    pub seal_number2: Option<String>,
    pub seal_number3: Option<String>,
    pub meter_manufacturer_other: Option<String>,
    pub meter_manufacturer_type_other: Option<String>,
    pub additional_photo1: Option<String>,
    pub additional_photo2: Option<String>,
    pub additional_photo1_comment: Option<String>,
    pub additional_photo2_comment: Option<String>,
}


impl  Enumeration {
    pub fn save_enumeration(data : Enumeration) -> i32{
        let device_id = 0;
        let mut new_enumeration = data;
        new_enumeration.photo_of_premise = Option::from(convertBase64ToFile());
        device_id
    }
}

fn convertBase64ToFile() -> String{
    String::new()
}
