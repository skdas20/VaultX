# How to Install VaultX on Windows

## Step 1: Prepare the File
1.  Download the **`vx-windows-x64.exe`** file to your computer.
2.  Rename the file to just **`vx.exe`** (this makes it easier to type).

## Step 2: Create a Folder
1.  Open File Explorer.
2.  Create a new folder where you want to keep the program.
    *   *Recommended:* `C:\VaultX`
    *   *Alternative:* `C:\Users\YourName\bin`
3.  Move **`vx.exe`** into this new folder.

## Step 3: Add to PATH (Crucial Step)
This allows you to run `vx` from any command prompt.

1.  Press the **Windows Key** and type **"env"**.
2.  Select **"Edit the system environment variables"**.
3.  Click the **"Environment Variables..."** button at the bottom right.
4.  In the **"System variables"** section (bottom half), scroll down and select **"Path"**, then click **"Edit..."**.
5.  Click **"New"** on the right side.
6.  Paste the path to your folder (e.g., `C:\VaultX`).
7.  Click **OK** on all open windows to save.

## Step 4: Verify Installation
1.  Open a **new** PowerShell or Command Prompt window (close any existing ones).
2.  Type the following command and press Enter:
    ```powershell
    vx --version
    ```
3.  If you see `VaultX 0.1.0`, you are all set!

## Troubleshooting
*   **"Term is not recognized":** This means Step 3 wasn't done correctly, or you didn't close and reopen your terminal window.
*   **Defender/Antivirus Warning:** Since this is a new tool not signed by Microsoft, Windows Defender might warn you. Click "More info" -> "Run anyway".
