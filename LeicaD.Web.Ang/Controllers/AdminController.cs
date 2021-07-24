using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using LeicaD.Web.Models;
using LeicaD.Web.Services;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace LeicaD.Web.Controllers
{
    [Authorize]
    [ApiController]
    [Route("[controller]")]
    public class AdminController : ControllerBase
    {
        private readonly ILogger<AdminController> _logger;
        private readonly IAdminUserService _adminService;


        public AdminController(ILogger<AdminController> logger, IAdminUserService adminUserService)
        {
            _logger = logger;
            _adminService = adminUserService;
        }

        [HttpGet]
        public IEnumerable<User> Get()
        {
            return _adminService.GetUsersAsync().Result;
        }
    }
}
