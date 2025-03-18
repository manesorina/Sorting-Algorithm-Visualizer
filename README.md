#Sorting Algorithm Visualizer

This is an interactive graphical application that visualizes the process of various common sorting algorithms. Users can select different sorting algorithms (such as Bubble Sort, Merge Sort, Quick Sort, etc.) and watch the sorting process step-by-step. The app allows users to control the sorting speed and reset the array for new visualizations.

Features:

Visualize multiple sorting algorithms (Bubble Sort, Merge Sort, Quick Sort, etc.)
Step-by-step sorting animations with adjustable speed
Real-time display of array elements as bars
Randomize the array or reset to start over
Clear and simple user interface with interactive controls
Technologies Used:

Rust: The programming language used for building the app.
eframe: A lightweight GUI framework built on egui for building the application interface.
egui: An immediate-mode GUI library used for drawing and handling the UI.
rand: Used to generate random arrays for sorting.
Multithreading: The sorting algorithms run in the background using std::thread to maintain a responsive UI.
