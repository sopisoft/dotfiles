# Reset conflicting ROS variables before switching to the Jazzy environment.
if [[ -f /opt/ros/jazzy/setup.zsh ]]; then
    if [[ -n ${ROS_DISTRO:-} && ${ROS_DISTRO:-} != "jazzy" ]]; then
        unset AMENT_PREFIX_PATH
        unset CMAKE_PREFIX_PATH
        unset COLCON_PREFIX_PATH
        unset LD_LIBRARY_PATH
        unset PKG_CONFIG_PATH
        unset PYTHONPATH
        unset ROS_DISTRO
        unset ROS_PYTHON_VERSION
        unset ROS_VERSION
    fi
    source /opt/ros/jazzy/setup.zsh
elif [[ -f /opt/ros/jazzy/setup.bash ]]; then
    if [[ -n ${ROS_DISTRO:-} && ${ROS_DISTRO:-} != "jazzy" ]]; then
        unset AMENT_PREFIX_PATH
        unset CMAKE_PREFIX_PATH
        unset COLCON_PREFIX_PATH
        unset LD_LIBRARY_PATH
        unset PKG_CONFIG_PATH
        unset PYTHONPATH
        unset ROS_DISTRO
        unset ROS_PYTHON_VERSION
        unset ROS_VERSION
    fi
    source /opt/ros/jazzy/setup.bash
fi
