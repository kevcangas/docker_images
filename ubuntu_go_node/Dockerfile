FROM ubuntu:latest

RUN apt-get update \
&& apt install -y software-properties-common \
&& add-apt-repository ppa:longsleep/golang-backports \
&& apt update \
&& apt install -y golang-go \
&& apt-get install -y openssh-server \ 
&& apt-get install -y nano \
&& apt-get install -y curl 

RUN mkdir /var/run/sshd
RUN echo 'root:root123' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed -i ' s/pattern/a\New line after pattern' filename

EXPOSE 22

RUN apt-get install git -y
RUN bash -c "$(curl -fsSL https://raw.githubusercontent.com/ohmybash/oh-my-bash/master/tools/install.sh)"

RUN apt-get install -y unzip
RUN curl -o- https://fnm.vercel.app/install | bash
RUN source /root/.bashrc
RUN fnm install 22

CMD ["/usr/sbin/sshd", "-D"]