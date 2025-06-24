use hcl_macros::Hcl;

#[derive(Hcl)]
#[hcl(block = "module", rename = "nested_module")]
struct ModuleConfig {
    #[hcl(label(index = 0))]
    module_name: String,

    #[hcl(attribute(rename = "source", optional = false))]
    module_source: String,

    #[hcl(attribute(optional = true))]
    version: Option<String>,

    #[hcl(attribute(optional = true, rename = "count"))]
    instance_count: Option<String>,
}

#[derive(Hcl)]
#[hcl(block = "locals")]
struct LocalsBlock {
    #[hcl(attribute(rename = "environment", optional = false))]
    env: String,

    #[hcl(attribute(rename = "project_name", optional = false))]
    project: String,

    #[hcl(attribute(optional = true))]
    region: Option<String>,
}

#[derive(Hcl)]
#[hcl(block = "provider")]
struct ProviderConfig {
    #[hcl(label(index = 0))]
    provider_name: String,

    #[hcl(attribute(optional = false))]
    region: String,

    #[hcl(attribute(optional = true))]
    access_key: Option<String>,

    #[hcl(attribute(optional = true))]
    secret_key: Option<String>,

    #[hcl(attribute(rename = "alias", optional = true))]
    provider_alias: Option<String>,
}

// Test struct with no labels, only attributes
#[derive(Hcl)]
#[hcl(block = "terraform")]
struct TerraformConfig {
    #[hcl(attribute(rename = "required_version", optional = false))]
    version: String,

    #[hcl(attribute(optional = true))]
    experiments: Option<String>,
}

// Test struct with complex naming
#[derive(Hcl)]
#[hcl(block = "output", rename = "terraform_output")]
struct OutputValue {
    #[hcl(label(index = 0))]
    output_name: String,

    #[hcl(attribute(optional = false))]
    value: String,

    #[hcl(attribute(optional = true))]
    description: Option<String>,

    #[hcl(attribute(optional = true))]
    sensitive: Option<String>,
}

#[cfg(test)]
mod advanced_tests {
    use super::*;

    #[test]
    fn test_module_with_optional_fields() {
        let module = ModuleConfig {
            module_name: "vpc".to_string(),
            module_source: "./modules/vpc".to_string(),
            version: Some("1.0.0".to_string()),
            instance_count: Some("3".to_string()),
        };

        let result = module.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("module"));
        assert!(hcl_string.contains("vpc"));
        assert!(hcl_string.contains("source"));
        assert!(hcl_string.contains("./modules/vpc"));
        assert!(hcl_string.contains("version"));
        assert!(hcl_string.contains("1.0.0"));
        assert!(hcl_string.contains("count"));
        assert!(hcl_string.contains("3"));
    }

    #[test]
    fn test_module_without_optional_fields() {
        let module = ModuleConfig {
            module_name: "database".to_string(),
            module_source: "./modules/rds".to_string(),
            version: None,
            instance_count: None,
        };

        let result = module.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("module"));
        assert!(hcl_string.contains("database"));
        assert!(hcl_string.contains("source"));
        assert!(hcl_string.contains("./modules/rds"));
        // Optional fields should not appear
        assert!(!hcl_string.contains("version"));
        assert!(!hcl_string.contains("count"));
    }

    #[test]
    fn test_locals_block() {
        let locals = LocalsBlock {
            env: "production".to_string(),
            project: "my-app".to_string(),
            region: Some("us-west-2".to_string()),
        };

        assert_eq!(LocalsBlock::hcl_block_type(), "locals");

        let result = locals.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("locals"));
        assert!(hcl_string.contains("environment"));
        assert!(hcl_string.contains("production"));
        assert!(hcl_string.contains("project_name"));
        assert!(hcl_string.contains("my-app"));
    }

    #[test]
    fn test_provider_with_all_fields() {
        let provider = ProviderConfig {
            provider_name: "aws".to_string(),
            region: "eu-west-1".to_string(),
            access_key: Some("AKIA123456".to_string()),
            secret_key: Some("secret123".to_string()),
            provider_alias: Some("eu".to_string()),
        };

        let result = provider.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("provider"));
        assert!(hcl_string.contains("aws"));
        assert!(hcl_string.contains("region"));
        assert!(hcl_string.contains("eu-west-1"));
        assert!(hcl_string.contains("access_key"));
        assert!(hcl_string.contains("secret_key"));
        assert!(hcl_string.contains("alias"));
        assert!(hcl_string.contains("eu"));
    }

    #[test]
    fn test_provider_minimal() {
        let provider = ProviderConfig {
            provider_name: "google".to_string(),
            region: "us-central1".to_string(),
            access_key: None,
            secret_key: None,
            provider_alias: None,
        };

        let result = provider.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("provider"));
        assert!(hcl_string.contains("google"));
        assert!(hcl_string.contains("region"));
        assert!(hcl_string.contains("us-central1"));
        // Optional fields should not appear
        assert!(!hcl_string.contains("access_key"));
        assert!(!hcl_string.contains("secret_key"));
        assert!(!hcl_string.contains("alias"));
    }

    #[test]
    fn test_terraform_config() {
        let terraform = TerraformConfig {
            version: ">=1.0".to_string(),
            experiments: Some("variable_validation".to_string()),
        };

        assert_eq!(TerraformConfig::hcl_block_type(), "terraform");

        let result = terraform.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("terraform"));
        assert!(hcl_string.contains("required_version"));
        assert!(hcl_string.contains(">=1.0"));
        assert!(hcl_string.contains("experiments"));
        assert!(hcl_string.contains("variable_validation"));
    }

    #[test]
    fn test_output_value() {
        let output = OutputValue {
            output_name: "vpc_id".to_string(),
            value: "aws_vpc.main.id".to_string(),
            description: Some("VPC ID".to_string()),
            sensitive: Some("false".to_string()),
        };

        assert_eq!(OutputValue::hcl_block_type(), "output");
        assert_eq!(OutputValue::hcl_name(), "terraform_output");

        let result = output.to_hcl();
        assert!(result.is_ok());

        let hcl_string = result.unwrap();
        assert!(hcl_string.contains("output"));
        assert!(hcl_string.contains("vpc_id"));
        assert!(hcl_string.contains("value"));
        assert!(hcl_string.contains("aws_vpc.main.id"));
        assert!(hcl_string.contains("description"));
        assert!(hcl_string.contains("VPC ID"));
    }

    #[test]
    fn test_serialization_format() {
        let module = ModuleConfig {
            module_name: "test".to_string(),
            module_source: "./test".to_string(),
            version: Some("1.0".to_string()),
            instance_count: None,
        };

        let hcl_output = module.to_hcl().unwrap();

        // Check basic format structure
        assert!(hcl_output.starts_with("module"));
        assert!(hcl_output.contains("{"));
        assert!(hcl_output.ends_with("}\n"));

        // Check that labels come before attributes
        let module_pos = hcl_output.find("module").unwrap();
        let test_pos = hcl_output.find("test").unwrap();
        let source_pos = hcl_output.find("source").unwrap();

        assert!(module_pos < test_pos);
        assert!(test_pos < source_pos);
    }

    #[test]
    fn test_complex_label_ordering() {
        let output = OutputValue {
            output_name: "database_endpoint".to_string(),
            value: "aws_rds_cluster.main.endpoint".to_string(),
            description: None,
            sensitive: Some("true".to_string()),
        };

        let hcl_output = output.to_hcl().unwrap();

        // Labels should appear immediately after block type
        assert!(hcl_output.contains("output \"database_endpoint\""));
        assert!(hcl_output.contains("value"));
        assert!(hcl_output.contains("sensitive"));
        assert!(!hcl_output.contains("description")); // Optional field with None
    }

    #[test]
    fn test_attribute_renaming_consistency() {
        let provider = ProviderConfig {
            provider_name: "azurerm".to_string(),
            region: "West Europe".to_string(),
            access_key: Some("test_key".to_string()),
            secret_key: Some("test_secret".to_string()),
            provider_alias: Some("westeurope".to_string()),
        };

        let hcl_output = provider.to_hcl().unwrap();

        // Check that field names are correctly renamed
        assert!(hcl_output.contains("alias")); // renamed from provider_alias
        assert!(!hcl_output.contains("provider_alias")); // original name should not appear
        assert!(hcl_output.contains("region")); // not renamed
        assert!(hcl_output.contains("access_key")); // not renamed
        assert!(hcl_output.contains("secret_key")); // not renamed
    }
}
