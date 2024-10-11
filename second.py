import tkinter as tk
from tkinter import PhotoImage

def create_second_frame(root, name):

    header = PhotoImage(file="images/header.png")
    header_pic = tk.Label(root, image=header)
    header_pic.image = header  # reference to avoid garbage collection
    header_pic.pack(side=tk.TOP, padx=5, pady=5)

    # Create a frame for the image and the label
    player_info = tk.Frame(root)
    player_info.pack(fill=tk.BOTH, expand=True)


    # Load the image
    player = PhotoImage(file="images/playercard.png")
    player_card = tk.Label(player_info, image=player)
    player_card.image = player  # Keep a reference to avoid garbage collection
    player_card.pack(side=tk.LEFT, padx=10, pady=10)

    # Create a label for the username
    user = tk.Label(player_info, text=f"Welcome, {name}!", font=("Baloo 2", 18)) 
    user.pack(side=tk.LEFT, padx=15, pady=5) # Added to make sure its on the image 
    user.place(relx=0.16, rely=0.50, anchor='center')  # Center the label over the image

    suspic = PhotoImage(file="images/suspicionchart.png")
    sus_pic = tk.Label(player_info, image=suspic)
    sus_pic.image = suspic  # reference to avoid garbage collection
    sus_pic.pack(side=tk.LEFT, padx=20, pady=10)

    # Add a button to go back
    another_button = tk.Button(root, text="Go Back", command=root.quit)
    another_button.pack(pady=20)
