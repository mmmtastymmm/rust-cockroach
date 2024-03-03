# Example rust project with docker cluster
Has the following:
* Cassandra docker compose
* Cockroach docker compose
* minio helm chart

## Minio Helm Chart
To run install the minio helm chart use the following command:
```bash
helm install minio minio-chart/
```
To forward relevant ports you can use kubectl
```bash
# For applications
kubectl port-forward pod/minio-0 9000:9000
# for the dashboard 
kubectl port-forward pod/minio-0 9001:9001
```