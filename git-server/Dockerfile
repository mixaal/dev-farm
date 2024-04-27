FROM alpine:latest

RUN apk add --no-cache openssh git # openrc

RUN addgroup -S git 
RUN adduser -S git -G git -s /usr/bin/git-shell

RUN X=$(tr -dc A-Za-z0-9 </dev/urandom | head -c 13); echo -n "git:$X" | chpasswd 


RUN ssh-keygen -A

RUN echo "RSAAuthentication yes" > /etc/ssh/sshd_config
RUN echo "PubkeyAuthentication yes" >> /etc/ssh/sshd_config
RUN echo "PasswordAuthentication no" >> /etc/ssh/sshd_config 
RUN echo "Subsystem	sftp	/usr/lib/ssh/sftp-server" >> /etc/ssh/sshd_config
RUN echo "AuthorizedKeysFile   /home/git/.ssh/authorized_keys" >> /etc/ssh/sshd_config

RUN rc-update add sshd
RUN mkdir  /home/git/.ssh
RUN chmod 0700 /home/git/.ssh

RUN echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIE/bWN808HmpIekHQAYISz3FAVp1m6CCeU768pzCRAcg mik@smurf" >> /home/git/.ssh/authorized_keys

RUN chmod 0600 /home/git/.ssh/authorized_keys

RUN  chown git:git /home/git/.ssh
RUN  chown git:git /home/git/.ssh/authorized_keys

COPY start.sh /
COPY target/release/git-mgmt /

CMD ["sh", "start.sh"]
