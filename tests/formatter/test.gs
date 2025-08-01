# Test various %define scenarios
%define SHORT hello                                                                     
world

%define VERY_LONG_CONSTANT_NAME this_is_a_very_long_replacement_text_that_exceeds_eighty_eight_characters 
more_content

%define MEDIUM_LENGTH_NAME some_reasonable_content                                      
continuation_line

%define SINGLE_LINE_CONSTANT 42

%define FUNCTION_MACRO(x, y)                                                            
    ((x) + (y))

%define NESTED_CONTENT                                                                  
    %include "other.gs"                                                                 
    more_stuff








# Test various %define scenarios
# %define SHORT hello \
# world

# %define VERY_LONG_CONSTANT_NAME this_is_a_very_long_replacement_text_that_exceeds_eighty_eight_characters \
# more_content

# %define MEDIUM_LENGTH_NAME some_reasonable_content \
# continuation_line

# %define SINGLE_LINE_CONSTANT 42

# %define FUNCTION_MACRO(x, y) \
#    ((x) + (y))

# %define NESTED_CONTENT \
#    %include "other.gs" \
#    more_stuff