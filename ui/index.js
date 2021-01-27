$(function()
{
  $("#dirInput").on("change input", function() {
      var path = $("#dirInput").prop("files")[0].path;
      //path = path.substring(0, path.lastIndexOf("\\"));
    $("#dirName").text(path);
  });

  $("#dirInput").on("dragenter", function() {
    $("dirDropzone").addClass("dragover");
  });

  $("#dirInput").on("dragleave", function() {
    $("dirDropzone").removeClass("dragover");
  });
});

// Need this access client's directories
const { dialog } = require("electron").remote;


/** 
 * Prompts user w/ file dialog, assigning the selected directory to the respective dirType.  
 **/
async function SelectDirectory(dirType)
{
    var folderPath = "";
    try
    {
        let options = { properties:["openDirectory"] };
        let folder = await dialog.showOpenDialog(options);
        folderPath = folder.filePaths[0];
    }
    catch (ex)
    {
        console.log("Error in SelectDirectory():\n" + ex);
    }
    
    switch (dirType)
    {
        case "input":
            console.log(dirType);
            break;
        
        case "output":
            console.log(dirType);
            break;

        default:
            console.log("Undefined directory type!");
            break;

    }
    console.log(folderPath);
}

/** 
 * Prompts user w/ file dialog.  
 **/
async function SelectFiles()
{
    try
    {
        let options = { properties:["multiSelections"] }
        let files = await dialog.showOpenDialog(options);
        files.filePaths.forEach(file => {
            console.log(file)
        });
    }
    catch (ex)
    {
        console.log("Error in SelectFiles():\n" + ex);
    }
}