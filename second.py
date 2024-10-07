import tkinter as tk
from tkinter import PhotoImage

def create_second_frame(root, name):
    # Create a frame for the image and the label
    main_frame = tk.Frame(root)
    main_frame.pack(fill=tk.BOTH, expand=True)

    # Load the image
    player = PhotoImage(file="images/playercard.png")
    image_label = tk.Label(main_frame, image=player)
    image_label.image = player  # Keep a reference to avoid garbage collection
    image_label.pack(side=tk.LEFT, padx=10, pady=10)

    # Create a label for the username
    username_label = tk.Label(main_frame, text=f"Welcome, {name}!", font=("Baloo 2", 18)) 
    username_label.place(relx=0.25, rely=0.25, anchor='center')  # Center the label over the image

    # Add a button to go back
    another_button = tk.Button(root, text="Go Back", command=root.quit)
    another_button.pack(pady=20)
