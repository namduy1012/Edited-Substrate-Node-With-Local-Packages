./target/release/nam-network build-spec --chain local > customSpec.json

sudo apt-get install moreutils jq

#!/bin/bash
value=$(jq -r '.bootNodes[0]' customSpec.json)

ips=($(hostname -I | cut -f1 -d' '))

for ip in "${ips[@]}"
do
    echo $ip
    cat customSpec.json | jq --arg new "${value/127.0.0.1/$ip}" '.bootNodes |= [$new]' | sponge customSpec.json
done

value=$(jq -r '.name' customSpec.json)
cat customSpec.json | jq --arg new "${value/Local Testnet/Nam-Network}" '.name |= $new' | sponge customSpec.json

value=$(jq -r '.id' customSpec.json)
cat customSpec.json | jq --arg new "${value/local_testnet/OMU}" '.id |= $new' | sponge customSpec.json

./target/release/nam-network build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json