[![CircleCI](https://circleci.com/gh/klaatu01/cachette.svg?style=svg)](https://circleci.com/gh/klaatu01/cachette)

# Cachette

AWS Lambda Extension for retreiving and caching of files on Startup.

## Support

Currently Supported Methods:
* [x] HTTP
* [x] S3 Files
* [x] S3 Buckets

Support for both:
* [x] x86_64
* [x] arm64

## Serverless Framework

If you are using the Serverless Framework checkout the official [plugin](github.com/klaatu01/serverless-plugin-cachette).

## Prebuilt Layers

Provided you are in using any of the following AWS Regions:
  - eu-west-1
  - eu-west-2
  - eu-central-1
  - us-east-1
  - us-east-2
  - us-west-2

There are some premade layers avaliable for each Destination and Architecture:
  - arn:aws:lambda:<region>:856198688143:layer:cachette_x86_64
  - arn:aws:lambda:<region>:856198688143:layer:cachette_arm64

## Building from source

If your organisation does not want to use one of the premade layers. Cachette can be built from source for your desired `architecture (x86_64|arm64)`.

### Dependencies
 - [docker](https://github.com/docker/cli): To compile the plugin on the target Architecture.
 - [aws-cli](https://github.com/aws/aws-cli): To deploy the layer to your AWS Account.
 
### Building:
```bash
./build.sh <x86_64|arm64>
```

### Publishing:
```bash
./publish.sh <x86_64|arm64> <region>
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
