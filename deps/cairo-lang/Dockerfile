# Use an official Python 3.9 slim image
FROM python:3.9-slim

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV CAIRO_VER="v0.13.3"

# Set working directory
WORKDIR /app

# Install system dependencies required for building
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    build-essential \
    libgmp-dev \
    curl \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Clone the release from Cairo-lang
RUN git clone --branch ${CAIRO_VER} --depth 1 https://github.com/starkware-libs/cairo-lang.git
# Copy requirements from SNOS project
COPY requirements.txt .

RUN pip3 install -r requirements.txt
RUN cairo-compile cairo-lang/src/starkware/starknet/core/os/os.cairo --output os_latest.json --cairo_path cairo-lang/src

# Default command
CMD ["bash"]

