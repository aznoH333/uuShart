use crate::information_gatherer::{information_resource::InformationResource, information_resource_type::InformationResourceType};


pub struct UnicornInformationResource{
    url: String
}

impl InformationResource for UnicornInformationResource{
    fn get_resource(&self) -> &String {
        return &self.url;
    }

    fn get_resource_type(&self) -> &InformationResourceType {
        return &InformationResourceType::UuKit;
    }
}

impl UnicornInformationResource{
    pub fn new(url: String) -> UnicornInformationResource{
        return UnicornInformationResource { url: url };
    }
}