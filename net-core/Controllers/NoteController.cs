using System;
using System.Linq;
using System.Collections;
using System.Collections.Concurrent;

using Microsoft.AspNetCore.Mvc;
using System.ComponentModel.DataAnnotations;


namespace netcore.Controllers;

[ApiController]
[Route("[controller]")]
public class NoteController : ControllerBase
{
    // NOTE: normal dictionary is not thread safe.
    private static readonly IDictionary<string, Note> userNotes = new ConcurrentDictionary <string, Note>();

    private readonly ILogger<NoteController> _logger;

    public NoteController(ILogger<NoteController> logger)
    {
        _logger = logger;
    }

    [HttpGet]
    public async Task<IActionResult> GetAll()
    {
        var userId = Request.Headers["userId"];
        if(String.IsNullOrWhiteSpace(userId)){
            return Unauthorized();
        }

        return Ok(new
        {
            items = userNotes
        });
    }

    [HttpGet("{noteId}")]
    public async Task<IActionResult> GetById(string noteId) 
    {
        var userId = Request.Headers["userId"];
        if(string.IsNullOrWhiteSpace(userId)){
            return Unauthorized();
        }

        if(!userNotes.ContainsKey(noteId)){
            return NotFound();
        }

        _logger.LogDebug("Note to return:", userNotes[noteId]);

        return Ok(new {
            item = userNotes[noteId]
        });
    }

    [HttpPost]
    public async Task<IActionResult> CreateNote(NewNoteDto dto) 
    {
        if(!ModelState.IsValid){
            return BadRequest();
        }

        var userId = Request.Headers["userId"];
        if(String.IsNullOrWhiteSpace(userId)){
            return Unauthorized();
        }

        _logger.LogDebug("NewNoteDto:", dto);


        var noteId = Guid.NewGuid().ToString();
        var note = new Note{
            Title = dto.Title,
            Content = dto.Content,
            CreatedAt = DateTime.Now
        };

        userNotes.Add(noteId, note);
        
        return Created(
            String.Format("/note/{0}", noteId),
            new { id = noteId }
        );
    }
}


public class Note {
    public string Title {get;set;}
	public string Content {get;set;}
	public DateTime CreatedAt {get;set;}
}

public class NewNoteDto {

    [Required]
    [MinLength(1)]
    public string Title {get;set;}
    
    [Required]
    [MinLength(1)]
    public string Content {get;set;}
}