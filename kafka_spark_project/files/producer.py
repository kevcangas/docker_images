import time
import json
import random
from kafka import KafkaProducer

# Configuración del Productor
producer = KafkaProducer(
    bootstrap_servers=['localhost:9092'],
    value_serializer=lambda x: json.dumps(x).encode('utf-8')
)

TOPIC_NAME = 'sensores_iot'

print(f"Iniciando simulación de sensores hacia el tópico: {TOPIC_NAME}")

try:
    while True:
        # Simular datos
        data = {
            'sensor_id': random.choice(['S1', 'S2', 'S3']),
            'temperatura': round(random.uniform(30.0, 35.0), 2),
            'timestamp': int(time.time())
        }
        
        # Enviar a Kafka
        producer.send(TOPIC_NAME, value=data)
        print(f"Enviado: {data}")
        
        time.sleep(1) # Un dato por segundo
except KeyboardInterrupt:
    print("Simulación detenida.")