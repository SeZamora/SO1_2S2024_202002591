#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/mm.h>
#include <linux/jiffies.h>
#include <linux/sched/signal.h>
#include <linux/slab.h>
#include <linux/cgroup.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Jorge");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y docker");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo_202002591"

static char* extract_container_id(const char *cgroup_name) {
    char *id = kmalloc(12, GFP_KERNEL); // Reservar espacio para el ID (12 caracteres)
    if (!id)
        return NULL;

    // Copiar el ID del cgroup
    strncpy(id, cgroup_name + 7, 12); // Copiar desde la posición 7 (después de "docker-")
    id[11] = '\0'; // Asegurarse de que sea una cadena nula terminada

    return id;
}

static int sysinfo_show(struct seq_file *m, void *v) {
    struct sysinfo si;
    struct task_struct *task;
    unsigned long rss, vsize, cpu_usage, mem_usage;
    unsigned long total_jiffies = jiffies;
    unsigned long process_jiffies;
    int first_process = 1;

    si_meminfo(&si);

    seq_printf(m, "{\n");
    seq_printf(m, "  \"Total RAM\": %lu,\n", si.totalram * 4);
    seq_printf(m, "  \"Free RAM\": %lu,\n", si.freeram * 4);
    seq_printf(m, "  \"RAM Uso\": %lu,\n", (si.totalram - si.freeram) * 4);
    seq_printf(m, "  \"Procesos\": [\n");

    for_each_process(task) {
        if (strstr(task->comm, "python")) {
            // Obtener información del proceso
            rss = get_mm_rss(task->mm) << (PAGE_SHIFT - 10);  
            vsize = task->mm->total_vm << (PAGE_SHIFT - 10); 
            process_jiffies = task->utime + task->stime;
            cpu_usage = (process_jiffies * 10000) / total_jiffies;  
            mem_usage = (rss * 10000) / si.totalram;

            // Obtener el cgroup del proceso
            char *cgroup_name = "N/A";
            struct cgroup *cgrp = task->cgroups->subsys[0]->cgroup;
            if (cgrp) {
                cgroup_name = cgrp->kn->name; // Obtener nombre del cgroup
            }

            if (!first_process) {
                seq_printf(m, ",\n");
            } else {
                first_process = 0;
            }

            // Extraer solo el ID del contenedor
            char *container_id = extract_container_id(cgroup_name);

            seq_printf(m, "    {\n");
            seq_printf(m, "      \"PID\": %d,\n", task->pid);
            seq_printf(m, "      \"Nombre\": \"%s\",\n", task->comm);
            seq_printf(m, "      \"Linea de Comando\": \"%s\",\n", container_id);
            seq_printf(m, "      \"Vsz\": %lu,\n", vsize);
            seq_printf(m, "      \"Rss\": %lu,\n", rss);
            seq_printf(m, "      \"Memoria Usada\": %lu.%02lu,\n", mem_usage / 100, mem_usage % 100);
            seq_printf(m, "      \"Cpu Usado\": %lu.%02lu\n", cpu_usage / 100, cpu_usage % 100);
            seq_printf(m, "    }");

            kfree(container_id); // Liberar la memoria del ID del contenedor
        }
    }

    seq_printf(m, "  ]\n");
    seq_printf(m, "}\n");

    return 0;
}

static int sysinfo_open(struct inode *inode, struct file *file) {
    return single_open(file, sysinfo_show, NULL);
}

static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
};

static int __init sysinfo_init(void) {
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "sysinfo module loaded\n");

    return 0;
}

static void __exit sysinfo_exit(void) {
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "sysinfo module unloaded\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);