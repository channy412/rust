apt-get update && \
apt-get -y upgrade && \
apt-get install -y build-essential && \
apt-get install -y software-properties-common && \
apt-get install -y byobu curl git htop man unzip vim wget && \
apt-get install -y ninja ssl pkg-config && \  

cp config.toml.verify config.toml
python x.py dist