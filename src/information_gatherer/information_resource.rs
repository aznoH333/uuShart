use super::information_resource_type::InformationResourceType;

pub trait InformationResource{
    fn get_resource(&self) -> &String;
    fn get_resource_type(&self) -> &InformationResourceType;
}