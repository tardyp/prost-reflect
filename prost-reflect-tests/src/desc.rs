use crate::test_file_descriptor;

#[test]
fn test_descriptor_methods() {
    let message_desc = test_file_descriptor()
        .get_message_by_name("my.package.MyMessage")
        .unwrap();
    assert_eq!(message_desc.name(), "MyMessage");
    assert_eq!(message_desc.full_name(), "my.package.MyMessage");
    assert_eq!(message_desc.parent_message(), None);
    assert_eq!(message_desc.package_name(), "my.package");
    assert_eq!(
        message_desc.reserved_ranges().flatten().collect::<Vec<_>>(),
        vec![2, 15, 9, 10, 11]
    );
    assert_eq!(
        message_desc.reserved_names().collect::<Vec<_>>(),
        vec!["foo", "bar"]
    );
    assert_eq!(message_desc.extension_ranges().count(), 0,);

    let field_desc = message_desc.get_field_by_name("my_field").unwrap();
    assert_eq!(field_desc.name(), "my_field");
    assert_eq!(field_desc.full_name(), "my.package.MyMessage.my_field");

    let nested_message_desc = test_file_descriptor()
        .get_message_by_name("my.package.MyMessage.MyNestedMessage")
        .unwrap();
    assert_eq!(nested_message_desc.name(), "MyNestedMessage");
    assert_eq!(
        nested_message_desc.full_name(),
        "my.package.MyMessage.MyNestedMessage"
    );
    assert_eq!(
        nested_message_desc.parent_message(),
        Some(message_desc.clone())
    );
    assert_eq!(nested_message_desc.package_name(), "my.package");

    let enum_desc = test_file_descriptor()
        .get_enum_by_name("my.package.MyEnum")
        .unwrap();
    assert_eq!(enum_desc.name(), "MyEnum");
    assert_eq!(enum_desc.full_name(), "my.package.MyEnum");
    assert_eq!(enum_desc.parent_message(), None);
    assert_eq!(enum_desc.package_name(), "my.package");
    assert_eq!(
        enum_desc.reserved_ranges().flatten().collect::<Vec<_>>(),
        vec![-2, 15, 9, 10, 11]
    );
    assert_eq!(
        enum_desc.reserved_names().collect::<Vec<_>>(),
        vec!["FOO", "BAR"]
    );

    let enum_value_desc = enum_desc.get_value_by_name("MY_VALUE").unwrap();
    assert_eq!(enum_value_desc.name(), "MY_VALUE");
    assert_eq!(enum_value_desc.full_name(), "my.package.MY_VALUE");

    let nested_enum_desc = test_file_descriptor()
        .get_enum_by_name("my.package.MyMessage.MyNestedEnum")
        .unwrap();
    assert_eq!(nested_enum_desc.name(), "MyNestedEnum");
    assert_eq!(
        nested_enum_desc.full_name(),
        "my.package.MyMessage.MyNestedEnum"
    );
    assert_eq!(nested_enum_desc.parent_message(), Some(message_desc));
    assert_eq!(nested_enum_desc.package_name(), "my.package");

    let service_desc = test_file_descriptor()
        .services()
        .find(|s| s.full_name() == "my.package.MyService")
        .unwrap();
    assert_eq!(service_desc.name(), "MyService");
    assert_eq!(service_desc.full_name(), "my.package.MyService");
    assert_eq!(service_desc.package_name(), "my.package");

    let method_desc = service_desc
        .methods()
        .find(|m| m.name() == "MyMethod")
        .unwrap();
    assert_eq!(method_desc.name(), "MyMethod");
    assert_eq!(method_desc.full_name(), "my.package.MyService.MyMethod");
}

#[test]
fn test_descriptor_methods_proto2() {
    let message_desc = test_file_descriptor()
        .get_message_by_name("my.package2.MyMessage")
        .unwrap();
    assert_eq!(message_desc.name(), "MyMessage");
    assert_eq!(message_desc.full_name(), "my.package2.MyMessage");
    assert_eq!(message_desc.parent_message(), None);
    assert_eq!(message_desc.package_name(), "my.package2");
    assert_eq!(
        message_desc
            .extension_ranges()
            .flatten()
            .collect::<Vec<_>>(),
        vec![100, 110, 111, 112, 113, 114, 115],
    );

    let mut extensions: Vec<_> = test_file_descriptor().all_extensions().collect();
    extensions.sort_by_key(|e| e.full_name().to_owned());
    assert_eq!(extensions.len(), 3);

    assert_eq!(
        extensions[0].full_name(),
        "my.package2.MyMessage.in_extendee"
    );
    assert_eq!(
        extensions[0].parent_message().unwrap().full_name(),
        "my.package2.MyMessage"
    );
    assert_eq!(
        extensions[0].containing_message().full_name(),
        "my.package2.MyMessage"
    );
    assert_eq!(
        extensions[0].json_name(),
        "[my.package2.MyMessage.in_extendee]"
    );

    assert_eq!(
        extensions[1].full_name(),
        "my.package2.OtherMessage.in_other"
    );
    assert_eq!(
        extensions[1].parent_message().unwrap().full_name(),
        "my.package2.OtherMessage"
    );
    assert_eq!(
        extensions[1].containing_message().full_name(),
        "my.package2.MyMessage"
    );
    assert_eq!(
        extensions[1].json_name(),
        "[my.package2.OtherMessage.in_other]"
    );

    assert_eq!(extensions[2].full_name(), "my.package2.in_file");
    assert!(extensions[2].parent_message().is_none());
    assert_eq!(
        extensions[2].containing_message().full_name(),
        "my.package2.MyMessage"
    );
    assert_eq!(extensions[2].json_name(), "[my.package2.in_file]");
}

#[test]
fn test_descriptor_names_no_package() {
    let message_desc = test_file_descriptor()
        .get_message_by_name("MyMessage")
        .unwrap();
    assert_eq!(message_desc.name(), "MyMessage");
    assert_eq!(message_desc.full_name(), "MyMessage");
    assert_eq!(message_desc.parent_message(), None);
    assert_eq!(message_desc.package_name(), "");

    let field_desc = message_desc.get_field_by_name("my_field").unwrap();
    assert_eq!(field_desc.name(), "my_field");
    assert_eq!(field_desc.full_name(), "MyMessage.my_field");

    let nested_message_desc = test_file_descriptor()
        .get_message_by_name("MyMessage.MyNestedMessage")
        .unwrap();
    assert_eq!(nested_message_desc.name(), "MyNestedMessage");
    assert_eq!(nested_message_desc.full_name(), "MyMessage.MyNestedMessage");
    assert_eq!(
        nested_message_desc.parent_message(),
        Some(message_desc.clone())
    );
    assert_eq!(nested_message_desc.package_name(), "");

    let enum_desc = test_file_descriptor().get_enum_by_name("MyEnum").unwrap();
    assert_eq!(enum_desc.name(), "MyEnum");
    assert_eq!(enum_desc.full_name(), "MyEnum");
    assert_eq!(enum_desc.parent_message(), None);
    assert_eq!(enum_desc.package_name(), "");

    let enum_value_desc = enum_desc.get_value_by_name("MY_VALUE").unwrap();
    assert_eq!(enum_value_desc.name(), "MY_VALUE");
    assert_eq!(enum_value_desc.full_name(), "MY_VALUE");

    let nested_enum_desc = test_file_descriptor()
        .get_enum_by_name("MyMessage.MyNestedEnum")
        .unwrap();
    assert_eq!(nested_enum_desc.name(), "MyNestedEnum");
    assert_eq!(nested_enum_desc.full_name(), "MyMessage.MyNestedEnum");
    assert_eq!(nested_enum_desc.parent_message(), Some(message_desc));
    assert_eq!(nested_enum_desc.package_name(), "");

    let service_desc = test_file_descriptor()
        .services()
        .find(|s| s.full_name() == "MyService")
        .unwrap();
    assert_eq!(service_desc.name(), "MyService");
    assert_eq!(service_desc.full_name(), "MyService");
    assert_eq!(service_desc.package_name(), "");

    let method_desc = service_desc
        .methods()
        .find(|m| m.name() == "MyMethod")
        .unwrap();
    assert_eq!(method_desc.name(), "MyMethod");
    assert_eq!(method_desc.full_name(), "MyService.MyMethod");
}