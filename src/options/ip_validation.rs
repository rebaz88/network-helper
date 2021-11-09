pub struct IPAddressValidation<'a> {
    pub is_valid: bool,
    pub message: &'a str,
}

pub fn is_valid_ip_address(ip: &str) -> IPAddressValidation {
    
    if ip.is_empty() {
        return IPAddressValidation {
            is_valid: false,
            message: "*** Ip address can not be empty! ***",
        }
    }
    
    for c in str::replace(ip, ".", "").chars() {
        if ! c.is_numeric() {
            return IPAddressValidation {
                is_valid: false,
                message: "*** Invalid character used in ip address! ***",
            }
        }
    }
    
    let parsed_ip: Vec<&str> = ip.split('.').collect();
    
    if parsed_ip.len() != 4 {
        return IPAddressValidation {
            is_valid: false,
            message: "*** Invalid ip address! ***",
        }
    } 
    
    return IPAddressValidation {
        is_valid: true,
        message: "Ip is valid",
    };
}

