# This scrtipt can interact with mikrotik and phpIPAM REST API

## Installation

Install Rust by [rustup](https://rustup.rs/)

## Usage 

```cargo run -- --config <path_to_your_config>``` 

or 

```<path_to_compiled_bin_file> --config <path_to_your_config>```

### config example

```
{
    "mikrotik_ip":"",
    "mikrotik_user":"",
    "mikrotik_password":"",
    "mikrotik_dhcp_server":"",
    "phpipam_proto_address":"",
    "phpipam_token":"",
    "phpipam_subnet_id":""
}
```

## Links

- [mikrotik REST API reference](https://help.mikrotik.com/docs/display/ROS/REST+API)
- [phpIPAM REST API](https://phpipam.net/api-documentation/)


## License

[MIT](https://choosealicense.com/licenses/mit/)
