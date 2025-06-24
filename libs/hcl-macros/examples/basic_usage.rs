use hcl_macros::Hcl;

#[derive(Hcl)]
#[hcl(block = "resource", rename = "aws_instance")]
struct AwsInstance {
    #[hcl(label(index = 0))]
    resource_type: String,

    #[hcl(label(index = 1))]
    resource_name: String,

    #[hcl(attribute(rename = "instance_type", optional = false))]
    instance_type: String,

    #[hcl(attribute(rename = "ami", optional = false))]
    ami: String,

    #[hcl(attribute(optional = true))]
    tags: Option<String>,
}

fn main() {
    let instance = AwsInstance {
        resource_type: "aws_instance".to_string(),
        resource_name: "web_server".to_string(),
        instance_type: "t2.micro".to_string(),
        ami: "ami-12345678".to_string(),
        tags: Some("Environment=Production".to_string()),
    };

    println!("Block type: {}", AwsInstance::hcl_block_type());
    println!("HCL name: {}", AwsInstance::hcl_name());

    match instance.to_hcl() {
        Ok(hcl_output) => {
            println!("Generated HCL:");
            println!("{}", hcl_output);
        }
        Err(e) => {
            println!("Error generating HCL: {}", e);
        }
    }
}
