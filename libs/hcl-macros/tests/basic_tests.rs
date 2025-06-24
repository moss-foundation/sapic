use hcl_macros::Hcl;

#[derive(Hcl)]
#[hcl(block = "resource", rename = "test_struct")]
struct TestStruct {
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

#[derive(Hcl)]
#[hcl(block = "variable")]
struct Variable {
    #[hcl(label(index = 0))]
    name: String,

    #[hcl(attribute(optional = false))]
    description: String,

    #[hcl(attribute(optional = false))]
    default: String,

    #[hcl(attribute(rename = "type", optional = false))]
    variable_type: String,
}

#[derive(Hcl)]
#[hcl(block = "data", rename = "data_source")]
struct DataSource {
    #[hcl(label(index = 0))]
    source_type: String,

    #[hcl(label(index = 1))]
    source_name: String,

    #[hcl(attribute(optional = false))]
    filter: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_struct_creation() {
        let test_struct = TestStruct {
            resource_type: "aws_instance".to_string(),
            resource_name: "example".to_string(),
            instance_type: "t2.micro".to_string(),
            ami: "ami-12345678".to_string(),
            tags: Some("production".to_string()),
        };

        assert_eq!(test_struct.resource_type, "aws_instance");
        assert_eq!(test_struct.resource_name, "example");
        assert_eq!(test_struct.instance_type, "t2.micro");
        assert_eq!(test_struct.ami, "ami-12345678");
        assert_eq!(test_struct.tags, Some("production".to_string()));
    }

    #[test]
    fn test_hcl_block_type() {
        assert_eq!(TestStruct::hcl_block_type(), "resource");
        assert_eq!(Variable::hcl_block_type(), "variable");
        assert_eq!(DataSource::hcl_block_type(), "data");
    }

    #[test]
    fn test_hcl_name() {
        assert_eq!(TestStruct::hcl_name(), "test_struct");
        assert_eq!(Variable::hcl_name(), "variable");
        assert_eq!(DataSource::hcl_name(), "data_source");
    }

    #[test]
    fn test_serialization() {
        let test_struct = TestStruct {
            resource_type: "aws_instance".to_string(),
            resource_name: "example".to_string(),
            instance_type: "t2.micro".to_string(),
            ami: "ami-12345678".to_string(),
            tags: Some("production".to_string()),
        };

        let result = test_struct.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("resource"));
        assert!(hcl_string.contains("aws_instance"));
        assert!(hcl_string.contains("example"));
        assert!(hcl_string.contains("instance_type"));
        assert!(hcl_string.contains("t2.micro"));
    }

    #[test]
    fn test_variable_serialization() {
        let variable = Variable {
            name: "instance_count".to_string(),
            description: "Number of instances".to_string(),
            default: "1".to_string(),
            variable_type: "number".to_string(),
        };

        let result = variable.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("variable"));
        assert!(hcl_string.contains("instance_count"));
        assert!(hcl_string.contains("description"));
        assert!(hcl_string.contains("Number of instances"));
    }

    #[test]
    fn test_optional_fields() {
        let test_struct_with_none = TestStruct {
            resource_type: "aws_instance".to_string(),
            resource_name: "example".to_string(),
            instance_type: "t2.micro".to_string(),
            ami: "ami-12345678".to_string(),
            tags: None,
        };

        let result = test_struct_with_none.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        // Optional field with None value should not appear in output
        assert!(!hcl_string.contains("tags"));
    }

    #[test]
    fn test_data_source_with_multiple_labels() {
        let data_source = DataSource {
            source_type: "aws_ami".to_string(),
            source_name: "ubuntu".to_string(),
            filter: "ubuntu/images/*".to_string(),
        };

        let result = data_source.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("data"));
        assert!(hcl_string.contains("aws_ami"));
        assert!(hcl_string.contains("ubuntu"));
        assert!(hcl_string.contains("filter"));
    }

    #[test]
    fn test_struct_with_renamed_attributes() {
        let test_struct = TestStruct {
            resource_type: "aws_instance".to_string(),
            resource_name: "web".to_string(),
            instance_type: "t3.medium".to_string(),
            ami: "ami-87654321".to_string(),
            tags: Some("test".to_string()),
        };

        let result = test_struct.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        // Should use renamed attribute names
        assert!(hcl_string.contains("instance_type"));
        assert!(hcl_string.contains("ami"));
        assert!(hcl_string.contains("t3.medium"));
        assert!(hcl_string.contains("ami-87654321"));
    }
}
