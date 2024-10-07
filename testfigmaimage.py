import tkinter as tk
from tkinter import messagebox, PhotoImage
import second  # Import the second file

# Create the main application window
root = tk.Tk()
root.title("Statistical Anti-Cheat")
root.geometry("400x300")  # Set the window size

photo = PhotoImage(file="images/Group 5.png")
image_label = tk.Label(root, image=photo)
image_label.pack(side=tk.TOP, padx=10, pady=10)

# Add a title label
#title_label = tk.Label(root, text="Welcome to My App", font=("Helvetica", 18))
#title_label.pack(pady=20)

# Add an entry field
entry_label = tk.Label(root, text="Enter your username:", font=("Baloo 2", 18))
entry_label.pack()

entry_field = tk.Entry(root, width=30)
entry_field.pack(pady=10)

# Function to handle button click
def on_button_click():
    name = entry_field.get()
    if name:
        messagebox.showinfo("Greetings", f"Hello, {name}!")
    else:
        messagebox.showwarning("Input Error", "Please enter your name.")

# Function to switch to the second frame
def switch_to_second():

    name = entry_field.get()  # Get the name from the entry field
    if not name:
        messagebox.showwarning("Input Error", "Please enter your name.")
        return
    
    # Clear the main window
    for widget in root.winfo_children():
        widget.destroy()
    
    # Create the second frame from the second module
    second.create_second_frame(root, name)

# Add a button to switch to the second frame
next_button = tk.Button(root, text="Next", command=switch_to_second)
next_button.pack(pady=20)

# Add a greet button
greet_button = tk.Button(root, text="Greet Me", command=on_button_click)
greet_button.pack(pady=20)

# Start the main loop
root.mainloop()

