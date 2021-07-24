using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using LeicaD.Web.Data;
using LeicaD.Web.Models;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;

namespace LeicaD.Web.Services
{

    public interface IAdminUserService
    {
        Task<User[]> GetUsersAsync();
        Task<IdentityRole[]> GetRolesAsync();
        Task<bool> UpdateUserRoleAsync(User u, string newRole);
    }
}
