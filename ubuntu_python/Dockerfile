FROM debian:latest

RUN apt-get update && apt-get install -y openssh-server && apt-get install -y nano
RUN mkdir /var/run/sshd
RUN echo 'root:root123' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

EXPOSE 22

RUN apt-get install curl -y
RUN apt-get install git -y
RUN bash -c "$(curl -fsSL https://raw.githubusercontent.com/ohmybash/oh-my-bash/master/tools/install.sh)"

CMD ["/usr/sbin/sshd", "-D"]