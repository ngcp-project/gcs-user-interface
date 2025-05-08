# With pyinstaller, run pyinstaller --onefile opencv.py, then move the file in dist/opencv to binaries folder
# After that, run this command: bun run target:triple, which adds your architecture to the binary name
# When running the program, go to http://127.0.0.1:5000/video_feed to see the camera

from flask import Flask, render_template, Response
import cv2

app = Flask(__name__)

cap = cv2.VideoCapture(0) # Might need to change this to 1 depending on your camera / OS

def gen_frames():  # generate frame by frame from camera
    while(cap.isOpened()):
        # Capture frame-by-frame
        ret, frame = cap.read()  # read the camera frame
        if ret:
            _, buffer = cv2.imencode('.jpg', frame)
            frame = buffer.tobytes()
            yield (b'--frame\r\n'
                   b'Content-Type: image/jpeg\r\n\r\n' + frame + b'\r\n')  # concat frame one by one and show result
        else:
            break
    cap.release()
    cv2.destroyAllWindows()

@app.route('/video_feed')
def video_feed():
    return Response(gen_frames(), mimetype='multipart/x-mixed-replace; boundary=frame')


@app.route('/')
def index():
    """Video streaming home page."""
    return render_template('index.html')


if __name__ == '__main__':
    app.run(debug=True)