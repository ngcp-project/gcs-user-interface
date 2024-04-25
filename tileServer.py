import http.server
import socketserver
import os

# Set the directory containing your tile files
DIRECTORY = 'public\Atlases\SLO'
# C:/Users/steve/Desktop/NGCP/Mobile Atlas Creator 2.3.3/atlases/SLO_Flyers_2024-04-24_015640/SLO

# Define the handler to serve files
class TileServer(http.server.SimpleHTTPRequestHandler):
    def translate_path(self, path):
        # Get the actual file path by joining the requested path with the base directory
        return os.path.join(DIRECTORY, path.lstrip('/'))

# Create a TCP server
with socketserver.TCPServer(("", 8001), TileServer) as httpd:
    print("Server started at http://localhost:8001")
    # Start the server
    httpd.serve_forever()

