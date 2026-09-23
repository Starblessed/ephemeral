FROM python:3.13-alpine

WORKDIR /app

COPY . .

# Install dependecies
RUN pip install -e .

EXPOSE 8411

CMD ["python", "-m", "ephemeral.main", "0.0.0.0", "8411"]
