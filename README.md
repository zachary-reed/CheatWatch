# CheatWatch

Group 9 - Zach Reed and Emily Montoya

This project is to create a Statistical Anti Cheat for Rainbow Six Siege.

## Recommendations

For a smooth and isolated development experience, it is **highly recommended** to run this project inside a virtual environment. This will help manage dependencies and avoid conflicts with system-wide packages.

## Setup

### 1. Create a Virtual Environment

To ensure the project runs in an isolated environment, create a virtual environment using one of the following methods:

#### Using `venv` (recommended for Python 3.x)

```bash
python -m venv venv
```

#### Using `virtualenv` (if installed globally)

```bash
virtualenv venv
```

### 2. Activate the Virtual Environment

Once the virtual environment is created, activate it using the appropriate command for your operating system:

- **Windows**:

  ```bash
  .\venv\Scripts\activate
  ```

- **Mac/Linux**:

  ```bash
  source venv/bin/activate
  ```

You should now see `(venv)` preceding the command prompt, indicating that the virtual environment is active.

### 3. Install Dependencies

Install all required dependencies listed in the `requirements.txt` file:

```bash
pip install -r requirements.txt
```

This will install all necessary packages for the project to run correctly.

### 4. Running the Project

Once the dependencies are installed, you can run the project as per the usual instructions in the project documentation.

---

## Notes

- To deactivate the virtual environment when you're done working, simply run:

  ```bash
  deactivate
  ```