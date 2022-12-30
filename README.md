 This is the README file for Notaboard. (github.com/notaboard/notaboard/)

 Notaboard is customised version of Nullboard, packed as Tauri application.
 So it is standalone desktop kanban board, running inside of webview for better safety,
 in case you don't trust your browser with your notes.

 Nullboard you can check out at : https://nullboard.io/preview

 Notaboard is in beta, so don't expect too much, but it works.
 Of course, there are some bugs.

 Notaboard consists of 2 parts : frontend part in /src directory,
 which is licensed under 2-clause BSD with the Commons Clause;
 and of Tauri backend in /src-tauri directory, which licensed under MIT.
 Both licenses are placed in their respective folders.

 How to install it ?

 0. git clone https://github.com/notaboard/notaboard

 1. Follow the common guide for setting up environment for Tauri :
    https://tauri.app/v1/guides/getting-started/prerequisites/
    Pick there specific sections for your OS.
    ( note : Notaboard is known to work on Linux, on other OSes we did not test. )

 2. Go to notaboard/src-tauri/src/ and open main.rs in text editor. Replace values
    of path1 and path0 to some directories on your system,
    where you would like to save your exported boards. These directories must be writable by your user.
    you can create them yourself if you wish so.

 3. Go back to the notaboard top directory.

 4. cargo tauri build

 5. In directory notaboard/src-tauri/target/release you shall see notaboard binary.

 6. Launch it and enjoy. Fresh installation gets "User Manual" board out of the box. it has "---" title.

 7. You can import "template" board from src/template directory, edit it up to your taste,
    rename to fit it's purpose and then export it from wheel menu. this is the way  to make new board.
    On next runs you should see all already imported boards inside top-right wheel menu as a list,
    where you can switch between boards for editing them.

 8. Notaboard supports some color marking :
    gggcolorggg -> color will be shown in green
    yyycoloryyy -> color will be shown as yellow on black background
    rrrcolorrrr -> color will be shown as red
    %%link.to/page%% -> will be shown as weblink "link.to/page".
 
