#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if Docker is installed
check_docker() {
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        exit 1
    fi

    if ! command -v docker-compose &> /dev/null; then
        print_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
}

# Function to stop and clean up existing containers
cleanup() {
    print_status "Cleaning up existing containers..."
    docker-compose down --volumes --remove-orphans 2>/dev/null || true
    docker container prune -f 2>/dev/null || true
    print_success "Cleanup completed"
}

# Function to build and start services using Docker Compose
start_with_compose() {
    print_status "Starting Icarus application with Docker Compose..."

    # Build and start services
    docker-compose up --build -d

    print_status "Waiting for services to be ready..."

    # Wait for database to be ready
    print_status "Waiting for PostgreSQL to be ready..."
    timeout=60
    while ! docker-compose exec postgres pg_isready -U icarususer -d icarusdb &>/dev/null; do
        sleep 2
        timeout=$((timeout - 2))
        if [ $timeout -le 0 ]; then
            print_error "PostgreSQL failed to start within 60 seconds"
            exit 1
        fi
    done
    print_success "PostgreSQL is ready!"

    # Wait for backend to be ready
    print_status "Waiting for backend to be ready..."
    timeout=60
    while ! curl -s http://localhost:8080 &>/dev/null; do
        sleep 2
        timeout=$((timeout - 2))
        if [ $timeout -le 0 ]; then
            print_error "Backend failed to start within 60 seconds"
            docker-compose logs backend
            exit 1
        fi
    done
    print_success "Backend is ready!"

    # Wait for frontend to be ready
    print_status "Waiting for frontend to be ready..."
    timeout=60
    while ! curl -s http://localhost:5173 &>/dev/null; do
        sleep 2
        timeout=$((timeout - 2))
        if [ $timeout -le 0 ]; then
            print_error "Frontend failed to start within 60 seconds"
            docker-compose logs frontend
            exit 1
        fi
    done
    print_success "Frontend is ready!"

    print_success "Icarus application is now running!"
    echo ""
    echo "🌐 Frontend (Svelte UI): http://localhost:5173"
    echo "🔗 Backend API: http://localhost:8080"
    echo "🗄️  PostgreSQL Database: localhost:5432"
    echo ""
    echo "Database credentials:"
    echo "  - Database: icarusdb"
    echo "  - User: icarususer"
    echo "  - Password: icaruspass"
    echo ""
    echo "To view logs: docker-compose logs -f"
    echo "To stop: docker-compose down"
}

# Function to build and start with single Dockerfile
start_with_single_container() {
    print_status "Starting Icarus application with single container..."

    # Build the image
    docker build -t icarus-app .

    # Stop existing container if running
    docker stop icarus-container 2>/dev/null || true
    docker rm icarus-container 2>/dev/null || true

    # Run the container
    docker run -d \
        --name icarus-container \
        -p 5173:5173 \
        -p 8080:8080 \
        -p 5432:5432 \
        icarus-app

    print_status "Waiting for services to start..."
    sleep 10

    print_success "Icarus application is starting!"
    echo ""
    echo "🌐 Frontend (Svelte UI): http://localhost:5173"
    echo "🔗 Backend API: http://localhost:8080"
    echo "🗄️  PostgreSQL Database: localhost:5432"
    echo ""
    echo "To view logs: docker logs -f icarus-container"
    echo "To stop: docker stop icarus-container"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Options:"
    echo "  start           Start the application using Docker Compose (recommended)"
    echo "  start-single    Start the application using single Docker container"
    echo "  stop            Stop the application"
    echo "  restart         Restart the application"
    echo "  logs            Show application logs"
    echo "  clean           Clean up containers and images"
    echo "  status          Show status of running containers"
    echo "  help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 start        # Start with Docker Compose"
    echo "  $0 stop         # Stop all services"
    echo "  $0 logs         # View logs"
}

# Function to stop services
stop_services() {
    print_status "Stopping Icarus application..."
    docker-compose down 2>/dev/null || true
    docker stop icarus-container 2>/dev/null || true
    docker rm icarus-container 2>/dev/null || true
    print_success "Application stopped"
}

# Function to show logs
show_logs() {
    if docker-compose ps | grep -q "Up"; then
        print_status "Showing Docker Compose logs..."
        docker-compose logs -f
    elif docker ps | grep -q icarus-container; then
        print_status "Showing single container logs..."
        docker logs -f icarus-container
    else
        print_warning "No running containers found"
    fi
}

# Function to show status
show_status() {
    print_status "Container status:"
    echo ""
    echo "Docker Compose services:"
    docker-compose ps 2>/dev/null || echo "No Docker Compose services running"
    echo ""
    echo "Single container:"
    docker ps --filter name=icarus-container --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}" 2>/dev/null || echo "No single container running"
}

# Function to clean everything
clean_all() {
    print_status "Cleaning up all Icarus containers and images..."
    docker-compose down --volumes --remove-orphans 2>/dev/null || true
    docker stop icarus-container 2>/dev/null || true
    docker rm icarus-container 2>/dev/null || true
    docker rmi icarus-app 2>/dev/null || true
    docker rmi icarusp_backend 2>/dev/null || true
    docker rmi icarusp_frontend 2>/dev/null || true
    docker volume prune -f 2>/dev/null || true
    docker system prune -f 2>/dev/null || true
    print_success "Cleanup completed"
}

# Main script logic
main() {
    # Check if Docker is installed
    check_docker

    case "${1:-start}" in
        "start")
            cleanup
            start_with_compose
            ;;
        "start-single")
            cleanup
            start_with_single_container
            ;;
        "stop")
            stop_services
            ;;
        "restart")
            stop_services
            sleep 2
            start_with_compose
            ;;
        "logs")
            show_logs
            ;;
        "status")
            show_status
            ;;
        "clean")
            clean_all
            ;;
        "help"|"-h"|"--help")
            show_usage
            ;;
        *)
            print_error "Unknown option: $1"
            echo ""
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
