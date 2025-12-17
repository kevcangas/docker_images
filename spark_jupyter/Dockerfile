FROM spark:latest

USER root

RUN apt-get update && \
    apt-get install -y \
        software-properties-common \
        openssh-server \
        nano \
        curl \
        git \
        python3-pip && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# --- CAMBIO 1: Añade py4j aquí ---
RUN pip install jupyterlab py4j

# --- CONFIGURACIÓN DE SSH ---
RUN mkdir /var/run/sshd
RUN echo 'root:root123' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# --- CONFIGURACIÓN DE SPARK ---
ENV SPARK_HOME=/opt/spark
ENV PYSPARK_PYTHON=python3
ENV PYSPARK_DRIVER_PYTHON=python3

# --- CAMBIO 2: Simplifica el PYTHONPATH ---
# Ya no necesitamos el truco del .zip de py4j
ENV PYTHONPATH=$SPARK_HOME/python:$PYTHONPATH

# --- EXPOSE Y CMD ---
EXPOSE 22
EXPOSE 9999

CMD ["jupyter", "lab", "--port=9999","--allow-root","--no-browser","--ip=0.0.0.0"]