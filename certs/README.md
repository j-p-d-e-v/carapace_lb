# Certificates

You can modify the certificates directory in the config.toml.

## Generate self-signed certificates.
```
openssl req -x509 -sha256 -days 356 -nodes -newkey rsa:2048 -subj "/CN=localhost.com/C=PH/L=Philippines" -keyout localhost.pem -out localhost.crt 
```
