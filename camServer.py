#111!/usr/bin/env python
# -*- coding: utf-8 -*

#sudo apt-get install python3-flask
#pip3 install opencv-python

from flask import Flask, render_template, Response
import cv2

app = Flask(__name__)
#app.config["CACHE_TYPE"] = "null"

@app.route('/')
def index():
    """Video streaming home page."""
    return render_template('index.html')

vs = cv2.VideoCapture(0)  # Camera 1
vs2 = cv2.VideoCapture(1)  # Camera 2
def gen(vs):
    """Video streaming generator function."""
    
    while True:
        ret,frame=vs.read()
        ret, jpeg = cv2.imencode('.jpg', frame)
        frame=jpeg.tobytes()
        yield (b'--frame\r\n'
        b'Content-Type: image/jpeg\r\n\r\n' + frame + b'\r\n')
        
    vs.release()
    cv2.destroyAllWindows() 

@app.route('/video_feed')
def video_feed():
    """Video streaming route. Put this in the src attribute of an img tag."""
    return Response(gen(vs),mimetype='multipart/x-mixed-replace; boundary=frame')

@app.route('/video_feed2')
def video_feed2():
    """Video streaming route for camera 2."""
    return Response(gen(vs2), mimetype='multipart/x-mixed-replace; boundary=frame')

if __name__ == '__main__': 
    app.run(host='0.0.0.0', port =5000, debug=True, threaded=True)